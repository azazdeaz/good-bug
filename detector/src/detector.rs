use std::fs;
use std::io::Read;
use std::sync::Arc;

use image::GenericImageView;
use tflite::ops::builtin::BuiltinOpResolver;
use tflite::{FlatBufferModel, InterpreterBuilder, Result};
use tokio::sync::{oneshot, Mutex};
use tokio_stream::StreamExt;

use common::{
    msg::{Broadcaster, Msg},
    settings::Settings,
    types::BoxDetection,
    utils::LastValue,
};

struct DetectionWorker {
    _thread: std::thread::JoinHandle<()>,
    _sender: std::sync::mpsc::Sender<(Vec<u8>, oneshot::Sender<Vec<BoxDetection>>)>,
}

impl DetectionWorker {
    fn new(detector_model: String) -> Self {
        let settings = Settings::new().unwrap();
        let buf = fs::read(detector_model).expect("Couldn't find the detector model");
        let model = FlatBufferModel::build_from_buffer(buf).unwrap();

        // TODO move these into settings
        let min_score = 0.5;
        let input_width: u32 = 320;
        let input_height: u32 = 320;

        let (sx, rx) = std::sync::mpsc::channel::<(Vec<u8>, oneshot::Sender<Vec<BoxDetection>>)>();

        let handler = std::thread::spawn(move || {
            let resolver = BuiltinOpResolver::default();
            let builder = InterpreterBuilder::new(model, &resolver).unwrap();
            let mut interpreter = builder.build().unwrap();

            interpreter.set_num_threads(3);

            interpreter.allocate_tensors().unwrap();

            let inputs = interpreter.inputs().to_vec();
            assert_eq!(inputs.len(), 1);

            let input_index = inputs[0];

            let outputs = interpreter.outputs().to_vec();
            assert_eq!(outputs.len(), 4);

            let output_index = outputs[0];

            let detection_boxes_idx = outputs[0];
            let detection_classes_idx = outputs[1];
            let detection_scores_idx = outputs[2];
            let num_detections_idx = outputs[3];

            let input_tensor = interpreter.tensor_info(input_index).unwrap();
            assert_eq!(
                input_tensor.dims,
                vec![1, input_height as usize, input_width as usize, 3]
            );

            let output_tensor = interpreter.tensor_info(output_index).unwrap();
            println!("output_tensor {:?}", output_tensor);
            assert_eq!(output_tensor.dims, vec![1, 25, 4]);

            while let Ok((img, response)) = rx.recv() {
                // input_file.read_exact(interpreter.tensor_data_mut(input_index)?)?;
                // let mut img = ImageReader::open(format!("data/{}.png", i)).unwrap().decode().unwrap().grayscale();
                // img.invert();
                let img =
                    image::load_from_memory_with_format(&img.as_slice(), image::ImageFormat::Jpeg)
                        .expect("Detector: failed to read frame as Jpeg");
                let (im_width, im_height) = img.dimensions();
                let img = img.resize_exact(
                    input_width,
                    input_height,
                    image::imageops::FilterType::Gaussian,
                );
                let img = img.to_bytes();
                let input = interpreter.tensor_data_mut(input_index).unwrap();
                let t = std::time::Instant::now();
                // let mut img = Cursor::new(img);
                img.as_slice().read(input).unwrap();
                // for i in 0..(320*320*3) {
                //     input[i] = img[i];//(img[i] as f32) / 127.5 - 1.0;
                // }
                println!("conversion time {:?}", t.elapsed());

                let t = std::time::Instant::now();
                interpreter.invoke().unwrap();
                println!("inference time {:?}", t.elapsed());

                // let output = interpreter.tensor_data(output_index).unwrap();
                let detection_boxes: &[f32] = interpreter.tensor_data(detection_boxes_idx).unwrap();
                let detection_classes: &[f32] =
                    interpreter.tensor_data(detection_classes_idx).unwrap();
                let detection_scores: &[f32] =
                    interpreter.tensor_data(detection_scores_idx).unwrap();
                let num_detections: &[f32] = interpreter.tensor_data(num_detections_idx).unwrap();
                // let guess = output.iter().enumerate().max_by(|x, y| x.1.cmp(y.1)).unwrap().0;

                // println!("detection_boxes {:?}", detection_boxes);
                // println!("detection_classes {:?}", detection_classes);
                // println!("detection_scores {:?}", detection_scores);
                // println!("num_detections {:?}", num_detections);

                let mut detections = Vec::new();
                for i in 0..num_detections[0] as usize {
                    if min_score > detection_scores[i] {
                        break;
                    }
                    let rect = &detection_boxes[i * 4..i * 4 + 4];
                    detections.push(BoxDetection {
                        ymin: rect[0] * im_height as f32,
                        xmin: rect[1] * im_width as f32,
                        ymax: rect[2] * im_height as f32,
                        xmax: rect[3] * im_width as f32,
                        score: detection_scores[i],
                        class: detection_classes[i] as u32,
                    });
                }
                println!("detections {:?}", detections);
                response.send(detections).ok();
            }
        });

        Self {
            _thread: handler,
            _sender: sx,
        }
    }
    fn detect(&self, img: Vec<u8>) -> oneshot::Receiver<Vec<BoxDetection>> {
        let (sx, rx) = oneshot::channel();
        self._sender.send((img, sx));
        rx
    }
}

// #[test]
// fn mobilenetv1_mnist() -> Result<()> {
//     test_mnist(&FlatBufferModel::build_from_file("data/MNISTnet_uint8_quant.tflite")?)?;

//     let buf = fs::read("data/MNISTnet_uint8_quant.tflite")?;
//     test_mnist(&FlatBufferModel::build_from_buffer(buf)?)
// }

pub struct Detector {}

impl Detector {
    pub fn run(broadcaster: &Broadcaster) {
        let detector_model = Settings::new().unwrap().detector_model;
        // bail if model is not set
        if detector_model.is_none() {
            return;
        }
        let detector_model = detector_model.unwrap();
        println!("Detector is running with {:?}", detector_model);

        let mut stream = broadcaster.stream().filter_map(|m| {
            if let Ok(Msg::Frame(frame)) = m {
                Some(frame)
            } else {
                None
            }
        });

        let worker = DetectionWorker::new(detector_model);
        let last_image = Arc::new(Mutex::new(LastValue::new()));

        {
            let last_image = Arc::clone(&last_image);
            tokio::spawn(async move {
                while let Some(frame) = stream.next().await {
                    let mut last_image = last_image.lock().await;
                    last_image.set(frame);
                }
            });
        };

        {
            let last_image = Arc::clone(&last_image);
            let publisher = broadcaster.publisher();
            tokio::spawn(async move {
                loop {
                    let last_image = last_image.lock().await.pop();
                    if let Some(frame) = last_image {
                        
                        let result = worker.detect(frame);
                        if let Ok(detections) = result.await {
                            publisher.send(Msg::Detections(detections)).ok();
                        }
                        // let detections =
                        //     tokio::task::spawn_blocking(|| detect(&model, frame).unwrap())
                        //         .await
                        //         .unwrap();
                    } else {
                        // wait if there is no new image to process
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    }
                }
            });
        }
    }
}
