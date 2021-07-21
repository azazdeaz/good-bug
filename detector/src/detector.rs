use std::convert::TryInto;
use std::fs::{self, File};
use std::io::{Read, Cursor};

use tflite::ops::builtin::BuiltinOpResolver;
use tflite::{FlatBufferModel, InterpreterBuilder, Result};
use image::{self, io::Reader as ImageReader, imageops::FilterType};
use tokio_stream::StreamExt;

use common::{settings::Settings, msg::{Msg, Broadcaster}};


fn detect(model: &FlatBufferModel, img: Vec<u8>) -> Result<()> {
    let resolver = BuiltinOpResolver::default();

    let builder = InterpreterBuilder::new(model, &resolver)?;
    let mut interpreter = builder.build()?;

    interpreter.set_num_threads(2);

    interpreter.allocate_tensors()?;

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
    assert_eq!(input_tensor.dims, vec![1, 320, 320, 3]);

    let output_tensor = interpreter.tensor_info(output_index).unwrap();
    println!("output_tensor {:?}", output_tensor);
    assert_eq!(output_tensor.dims, vec![1, 25, 4]);


    // input_file.read_exact(interpreter.tensor_data_mut(input_index)?)?;
    // let mut img = ImageReader::open(format!("data/{}.png", i)).unwrap().decode().unwrap().grayscale();
    // img.invert();
    let img = image::load_from_memory_with_format(image::ImageFormat::Jpeg).expect("Detector: failed to read frame as Jpeg");
    let img = img.to_bytes();
    let input = interpreter.tensor_data_mut(input_index)?;
    let t = std::time::Instant::now();
    // let mut img = Cursor::new(img);
    img.as_slice().read(input).unwrap();
    // for i in 0..(320*320*3) {
    //     input[i] = img[i];//(img[i] as f32) / 127.5 - 1.0;
    // }
    println!("conversion time {:?}", t.elapsed());

    let t = std::time::Instant::now();
    interpreter.invoke()?;
    println!("inference time {:?}", t.elapsed());

    // let output = interpreter.tensor_data(output_index)?;
    let detection_boxes: &[f32] = interpreter.tensor_data(detection_boxes_idx)?;
    let detection_classes: &[f32] = interpreter.tensor_data(detection_classes_idx)?;
    let detection_scores: &[f32] = interpreter.tensor_data(detection_scores_idx)?;
    let num_detections: &[f32] = interpreter.tensor_data(num_detections_idx)?;
    // let guess = output.iter().enumerate().max_by(|x, y| x.1.cmp(y.1)).unwrap().0;

    // println!("detection_boxes {:?}", detection_boxes);
    // println!("detection_classes {:?}", detection_classes);
    // println!("detection_scores {:?}", detection_scores);
    // println!("num_detections {:?}", num_detections);   
    Ok(())
}

// #[test]
// fn mobilenetv1_mnist() -> Result<()> {
//     test_mnist(&FlatBufferModel::build_from_file("data/MNISTnet_uint8_quant.tflite")?)?;

//     let buf = fs::read("data/MNISTnet_uint8_quant.tflite")?;
//     test_mnist(&FlatBufferModel::build_from_buffer(buf)?)
// }


pub struct Detector {}

impl Detector {
    pub fn new(broadcaster: &Broadcaster) {
        let stream = broadcaster.stream()
            .filter_map(|m| {
                if let Ok(Msg::Frame(frame)) = m { Some(frame) } else { None }
                
            });

        let (sx, rx) = tokio::sync::watch::channel(None);
        tokio::spawn(async move {
            while let Some(frame) = stream.next().await { 
                sx.send(Some(frame)).ok();
            }
        });
        tokio::spawn(async move {
            loop {
                if let Ok(_) = rx.changed().await { 
                    if let Some(frame) = *rx.borrow() {
                        tokio::task::spawn_blocking(move || {
                            let settings = Settings::new().unwrap();
                            let buf = fs::read(settings.detecor_model)?;
                            detect(&FlatBufferModel::build_from_buffer(buf)?, frame)
                        }).await.unwrap();
                    }
                }
            }
        });
    }
}
