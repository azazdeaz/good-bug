use gdnative::api::*;
use gdnative::prelude::*;

/// The Game "class"
#[derive(NativeClass)]
#[inherit(Node)]
// #[register_with(Self::register_builder)]
#[register_with(Self::register_signals)]

pub struct Game {
    name: String,
    values: Values,
    // pub_vel: zmq::Socket,
    // req_slam: zmq::Socket,
    rx: Option<Receiver<Incoming>>,
    sub_tf_gt: Option<rosrust::Subscriber>,
    navigator: navigator::Navigator,
    client: grpc_client::GrpcClient,
}

enum Incoming {
    OpenVSlamPB(items::VSlamMap),
    GroundTruthPose(na::Isometry3<f64>),
}

use na::UnitQuaternion;
use prost::Message;
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/map_segment.rs"));
}
use std::collections::HashMap;

extern crate yaml_rust;

use hello_world::greeter_client::GreeterClient;
use hello_world::{HelloRequest, Speed};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}


// #[derive(PartialEq, Clone)]
// struct Pose {
//     rotation: Array2<f64>,
//     translation: Array2<f64>,
// }
struct StepMark {
    time: time::SystemTime,
    pose: na::Isometry3<f64>,
}
impl StepMark {
    fn should_move(&self, pose: &na::Isometry3<f64>) -> bool {
        pose != &self.pose || self.time.elapsed().unwrap().as_secs() > 1
    }
}
struct Values {
    max_lm_obs: u32,
    min_lm_obs: u32,
    landmarks: HashMap<u32, (Vector3, Color, u32)>,
    keyframes: HashMap<u32, Vec<Vector3>>,
    current_frame: Option<Vec<Vector3>>,
    last_step_mark: StepMark,
    target: (f64, f64, f64),
    follow_target: bool,
    speed: Option<(f64, f64)>,
    step: Option<(f64, f64, f64)>,
    tracker_state: TrackerState,
    marked_keyframe: Option<u32>,
    camera_pose: Option<na::Isometry3<f64>>,
    ground_truth_pose: Option<Transform>,
    first_ground_truth_pose: Option<na::Isometry3<f64>>,
    scale_factor: f64,
    slam_scale: f64,
}

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::spawn;
use std::{
    sync::{Arc, Mutex},
    thread, time,
};
// use tungstenite::{connect, Error, Message};
use url::Url;
use zmq;

use crate::navigator;
use crate::types::*;
use crate::grpc_client;

extern crate nalgebra as na;

use ndarray::prelude::*;
use ndarray::{stack, Array, Axis};

use scarlet::colormap::ListedColorMap;

mod Colors {
    use gdnative::prelude::Color;

    pub struct C {
        r: i32,
        g: i32,
        b: i32,
    }
    impl C {
        pub fn as_godot(&self) -> Color {
            Color::rgb(
                self.r as f32 / 255.,
                self.g as f32 / 255.,
                self.b as f32 / 255.,
            )
        }
    }

    pub const FRAME: C = C {
        r: 0xe7,
        g: 0x83,
        b: 0xfc,
    };
    pub const EDGE: C = C {
        r: 0x63,
        g: 0x92,
        b: 0xff,
    };
    pub const CURRENT_FRAME: C = C {
        r: 0xff,
        g: 0x77,
        b: 0x5e,
    };
    pub const LANDMARK1: C = C {
        r: 0x1c,
        g: 0xff,
        b: 0x9f,
    };
    pub const LANDMARK2: C = C {
        r: 0x96,
        g: 0xff,
        b: 0x08,
    };
}

// #[path = "protos/map_segment.rs"]
// mod map_segment;
// use map_segment::Map;
// use protobuf;

// keyframe_vertices();

#[derive(Default)]
struct Wireframes {
    _zero_keyframe: Option<Array<f64, Ix2>>,
}
impl Wireframes {
    fn zero_keyframe(&mut self, rotation: &Array2<f64>, translation: &Array2<f64>) -> Array2<f64> {
        let zero_keyframe = self._zero_keyframe.get_or_insert_with(|| {
            let scale = 0.1;
            let f = 1.0 * scale;
            let cx = 2.0 * scale;
            let cy = 1.0 * scale;
            let c = array![0.0, 0.0, 0.0];
            let tl = array![-cx, cy, f];
            let tr = array![cx, cy, f];
            let br = array![cx, -cy, f];
            let bl = array![-cx, -cy, f];
            stack![Axis(1), c, tl, tr, c, tr, br, c, br, bl, c, bl, tl]
        });

        rotation.dot(zero_keyframe) + translation
    }
}

const W: Wireframes = Wireframes {
    _zero_keyframe: None,
};

fn mat44_to_vertices(pose: &items::v_slam_map::Mat44) -> (Vec<Vector3>, Array2<f64>, Array2<f64>) {
    let pose = pose.pose.to_vec();
    let pose = Array::from_vec(pose).into_shape((4, 4)).unwrap();
    let pose = inv_pose(pose);
    let rotation = pose.slice(s![0..3, 0..3]).to_owned();
    let translation = pose.slice(s![0..3, 3..4]).to_owned();

    let vertices = W.zero_keyframe(&rotation, &translation);
    let vertices = vertices.axis_iter(Axis(1));
    let vertices: Vec<Vector3> = vertices
        .map(|v| Vector3::new(v[0] as f32, v[1] as f32, v[2] as f32))
        .collect();
    (vertices, rotation, translation)
}

fn mat44_to_isometry3(pose: &items::v_slam_map::Mat44) -> na::Isometry3<f64> {
    let d = pose.pose.to_vec();
    let translation = na::Translation3::new(d[3], d[7], d[11]);
    let rotation = na::Matrix3::new(d[0], d[1], d[2], d[4], d[5], d[6], d[8], d[9], d[10]);
    let rotation = na::Rotation3::from_matrix(&rotation);
    let rotation = UnitQuaternion::from_rotation_matrix(&rotation);
    // let rotation = UnitQuaternion::from_basis_unchecked(&[
    //     na::Vector3::new(d[0], d[1], d[2]),
    //     na::Vector3::new(d[4], d[5], d[6]),
    //     na::Vector3::new(d[8], d[9], d[10]),
    // ]);
    na::Isometry3::from_parts(translation, rotation)
}

pub fn angle_difference(bearing1: f64, bearing2: f64) -> f64 {
    let pi = std::f64::consts::PI;
    let pi2 = pi * 2.;
    let diff = (bearing2 - bearing1) % pi2;
    if diff < -pi {
        pi2 + diff
    } else if diff > pi {
        -pi2 + diff
    } else {
        diff
    }
}

fn iso3_to_gd(iso: &na::Isometry3<f64>) -> Transform {
    let origin = Vector3::new(
        iso.translation.x as f32,
        iso.translation.y as f32,
        iso.translation.z as f32,
    );
    let r = iso.rotation.to_rotation_matrix();
    let basis = Basis::from_elements([
        Vector3::new(r[(0, 0)] as f32, r[(0, 1)] as f32, r[(0, 2)] as f32),
        Vector3::new(r[(1, 0)] as f32, r[(1, 1)] as f32, r[(1, 2)] as f32),
        Vector3::new(r[(2, 0)] as f32, r[(2, 1)] as f32, r[(2, 2)] as f32),
    ]);
    Transform { origin, basis }
}

// NOTE: I have no idea what im doing...
fn inv_pose(pose: Array2<f64>) -> Array2<f64> {
    let mut res = Array::zeros((4, 4));

    // let t = pose.slice(s![0..3,3..4])
    // res.slice(s![0..3,3]).assign(-pose.slice(s![0,0..3]) * pose[[0,3]]
    //     - pose.slice(s![1,0..3]) * pose[[1,3]]
    //     - pose.slice(s![2,0..3]) * pose[[2,3]]);

    // res.slice(s![0..3,0..3]).assign(pose.slice(s![0..3,0..3]).t());
    // res

    res[[0, 3]] =
        -pose[[0, 0]] * pose[[0, 3]] - pose[[1, 0]] * pose[[1, 3]] - pose[[2, 0]] * pose[[2, 3]];
    res[[1, 3]] =
        -pose[[0, 1]] * pose[[0, 3]] - pose[[1, 1]] * pose[[1, 3]] - pose[[2, 1]] * pose[[2, 3]];
    res[[2, 3]] =
        -pose[[0, 2]] * pose[[0, 3]] - pose[[1, 2]] * pose[[1, 3]] - pose[[2, 2]] * pose[[2, 3]];
    res[[0, 0]] = pose[[0, 0]];
    res[[0, 1]] = pose[[1, 0]];
    res[[0, 2]] = pose[[2, 0]];
    res[[1, 0]] = pose[[0, 1]];
    res[[1, 1]] = pose[[1, 1]];
    res[[1, 2]] = pose[[2, 1]];
    res[[2, 0]] = pose[[0, 2]];
    res[[2, 1]] = pose[[1, 2]];
    res[[2, 2]] = pose[[2, 2]];
    res

    // function inv(pose) {
    //     let res = new Array();
    //     for (let i = 0; i < 3; i++) {
    //         res.push([0, 0, 0, 0]);
    //     }
    //     // - R^T * t
    //     res[0][3] = - pose[0][0] * pose[0][3] - pose[1][0] * pose[1][3] - pose[2][0] * pose[2][3];
    //     res[1][3] = - pose[0][1] * pose[0][3] - pose[1][1] * pose[1][3] - pose[2][1] * pose[2][3];
    //     res[2][3] = - pose[0][2] * pose[0][3] - pose[1][2] * pose[1][3] - pose[2][2] * pose[2][3];
    //     res[0][0] = pose[0][0]; res[0][1] = pose[1][0]; res[0][2] = pose[2][0];
    //     res[1][0] = pose[0][1]; res[1][1] = pose[1][1]; res[1][2] = pose[2][1];
    //     res[2][0] = pose[0][2]; res[2][1] = pose[1][2]; res[2][2] = pose[2][2];

    //     return res;
    // }
}

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl Game {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
        let x = env!("OUT_DIR");
        godot_print!("Game builder is registered!");
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "current_frame",
            // Argument list used by the editor for GUI and generation of GDScript handlers. It can be omitted if the signal is only used from code.
            args: &[SignalArgument {
                name: "data",
                default: Variant::from_array(&VariantArray::new_shared()),
                export_info: ExportInfo::new(VariantType::Vector3Array),
                usage: PropertyUsage::DEFAULT,
            }],
        });

        builder.add_signal(Signal {
            name: "message",
            // Argument list used by the editor for GUI and generation of GDScript handlers. It can be omitted if the signal is only used from code.
            args: &[SignalArgument {
                name: "data",
                default: Variant::from_str(""),
                export_info: ExportInfo::new(VariantType::GodotString),
                usage: PropertyUsage::DEFAULT,
            }],
        });
    }

    /// The "constructor" of the class.
    fn new(_owner: &Node) -> Self {
        godot_print!("Game is created!!!");

        // let context = zmq::Context::new();
        // let publisher = context.socket(zmq::PUB).unwrap();
        // publisher
        //     .bind("tcp://*:5567")
        //     .expect("failed binding publisher");
        // let context = zmq::Context::new();
        // let req_slam = context.socket(zmq::REQ).unwrap();
        // req_slam
        //     .connect("tcp://192.168.50.19:5561")
        //     .expect("failed connecting requester");
        godot_print!("building game");
        let game = Game {
            name: "".to_string(),
            values: Values {
                max_lm_obs: 1,
                min_lm_obs: 0,
                landmarks: HashMap::new(),
                keyframes: HashMap::new(),
                current_frame: Some(vec![Vector3::new(0., 0., 0.)]),
                last_step_mark: StepMark {
                    time: time::SystemTime::now(),
                    pose: na::Isometry3::<f64>::identity(),
                },
                target: (0., 0., 0.),
                follow_target: false,
                speed: None,
                step: Some((0., 0., 0.)),
                tracker_state: TrackerState::NotInitialized,
                marked_keyframe: None,
                camera_pose: None,
                ground_truth_pose: None,
                first_ground_truth_pose: None,
                scale_factor: 1.0,
                slam_scale: 1.0,
            },
            // pub_vel: publisher,
            // req_slam,
            rx: None,
            sub_tf_gt: None,
            navigator: navigator::Navigator::new(),
            client: grpc_client::GrpcClient::new(),
        };
        godot_print!("build game");
        game
    }

    #[export]
    fn set_target(&mut self, _owner: TRef<Node>, x: f64, y: f64, z: f64) {
        self.values.speed = None;
        self.values.target = (x, y, z);
        let _ = self
            .navigator
            .send_target_pose
            .try_send(na::Isometry3::<f64>::from_parts(
                na::Translation3::new(x, y, z),
                na::UnitQuaternion::identity(),
            ));
    }

    #[export]
    fn set_follow_target(&mut self, _owner: TRef<Node>, on: bool) {
        godot_print!("follow_target {}", on);
        self.values.follow_target = on;
        self.navigator.send_self_drive_enabled.send(on);
    }

    #[export]
    fn set_speed(&mut self, _owner: TRef<Node>, left: f64, right: f64) {

        println!("-REQUESTING {} {}", left, right);
        let left = (left * 100.).floor() / 100.;
        let right = (right * 100.).floor() / 100.;
        // self.navigator.send_teleop_speed.send((left, right));
        // self.values.speed = Some((left, right));
        self.client.set_speed(left, right);
    }

    #[export]
    fn set_step(&mut self, _owner: TRef<Node>, left: f64, right: f64, time: f64) {
        self.values.step = Some((left, right, time));
    }

    #[export]
    fn set_scale_factor(&mut self, _owner: TRef<Node>, scale: f64) {
        self.values.scale_factor = scale;
    }

    #[export]
    fn set_min_lm_obs(&mut self, _owner: TRef<Node>, min_lm_obs: f64) {
        self.values.min_lm_obs = min_lm_obs as u32;
    }

    #[export]
    fn toggle_connection(&mut self, _owner: TRef<Node>, on: bool) {
        let msg = if on {
            "start_publisher"
        } else {
            "pause_publisher"
        };
        godot_print!("tell {}", msg);
        // self.req_slam
        //     .send(&msg, 0)
        //     .expect("failed to send cmd");
        // let response = self.req_slam.recv_msg(0).unwrap();
        // println!("Received reply {}", response.as_str().unwrap());
    }

    #[export]
    fn request_slam_terminate(&mut self, _owner: TRef<Node>) {
        godot_print!("request terminate");
        // self.req_slam
        //     .send(&"terminate", 0)
        //     .expect("failed to send cmd");
        // let response = self.req_slam.recv_msg(0).unwrap();
        // println!("Received reply {}", response.as_str().unwrap());
    }

    #[export]
    fn estimate_scale(&mut self, _owner: TRef<Node>) {
        let z = self
            .values
            .landmarks
            .values()
            .map(|v| v.0.y)
            .filter(|z| z.is_sign_positive());
        // .collect::<Vec<f32>>();

        let z = ndarray::Array::from_iter(z);
        let ground_level = z.mean();
        // let z = na::VectorN::from_vec(z);
        godot_print!("mean {:?}", ground_level);

        if let Some(ground_level) = ground_level {
            unsafe {
                let t = Transform::translate(Vector3::new(0.0, -ground_level, 0.0));
                _owner
                    .find_node("Ground", true, true)
                    .unwrap()
                    .assume_safe()
                    .cast::<CSGMesh>()
                    .unwrap()
                    .set_translation(Vector3::new(0.0, -ground_level, 0.0));

                self.values.slam_scale = ground_level as f64 / navigator::RobotBody::get_cam_height();
                self.navigator.send_slam_scale.send(self.values.slam_scale).unwrap();
            }
        }
    }


    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are "attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    unsafe fn _ready(&mut self, _owner: TRef<Node>) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        self.name = "Game".to_string();
        godot_print!("{} is ready!!!mak", self.name);

        let context = zmq::Context::new();

        let (tx, rx): (Sender<Incoming>, Receiver<Incoming>) = mpsc::channel();
        self.rx = Some(rx);
        let thread_tx = tx.clone();

        spawn(move || {
            loop {
                println!("Connecting...");
                // let connection = connect(Url::parse("ws://localhost:3012/socket").unwrap());
                let subscriber = context.socket(zmq::SUB).unwrap();
                subscriber
                    .connect("tcp://192.168.50.19:5566")
                    .expect("failed connecting subscriber");
                subscriber.set_subscribe(b"").expect("failed subscribing");
                loop {
                    let envelope = subscriber
                        .recv_string(0)
                        .expect("failed receiving envelope")
                        .unwrap();
                    let message = subscriber.recv_bytes(0).expect("failed receiving message");
                    // println!("{:?}", message);
                    let message = ::base64::decode(message).unwrap();
                    let msg = items::VSlamMap::decode(&mut std::io::Cursor::new(message)).unwrap();

                    // println!("{:?}", msg);
                    // let m = Map::parse_from_bytes(&message);
                    // m.merge_from(CodedInputStream::from_bytes(&message));
                    // println!("{:?}", m);
                    // let msg = format!("[{}] {}", envelope, message);
                    thread_tx.send(Incoming::OpenVSlamPB(msg)).unwrap();
                }
            }
        });

        fn find_node<T: SubClass<Node>>(owner: TRef<Node>, mask: String) -> TRef<T> {
            unsafe {
                owner
                    .find_node(mask, true, true)
                    .unwrap()
                    .assume_safe()
                    .cast::<T>()
                    .unwrap()
            }
        }

        find_node::<CheckButton>(_owner, "ConnectionBtn".into())
            .connect(
                "toggled",
                _owner,
                "toggle_connection",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        _owner
            .find_node("ScaleSlider", true, true)
            .unwrap()
            .assume_safe()
            .cast::<HSlider>()
            .unwrap()
            .connect(
                "value_changed",
                _owner,
                "set_scale_factor",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        _owner
            .find_node("MinLandmarks", true, true)
            .unwrap()
            .assume_safe()
            .cast::<HSlider>()
            .unwrap()
            .connect(
                "value_changed",
                _owner,
                "set_min_lm_obs",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        _owner
            .find_node("EstimateScaleBtn", true, true)
            .unwrap()
            .assume_safe()
            .cast::<Button>()
            .unwrap()
            .connect(
                "pressed",
                _owner,
                "estimate_scale",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        // rosrust::init("listener");

        // let thread_tx2 = tx.clone();
        // // Create subscriber
        // // The subscriber is stopped when the returned object is destroyed
        // self.sub_tf_gt = Some(
        //     rosrust::subscribe(
        //         "/X1/pose_static",
        //         100,
        //         move |v: msg::tf2_msgs::TFMessage| {
        //             // Callback for handling received messages
        //             // godot_print!("Received: {:?}", v);
        //             for t in v.transforms {
        //                 godot_print!("{} -> {}", t.header.frame_id, t.child_frame_id);
        //                 if t.child_frame_id == "X1" {
        //                     let origin = na::Translation3::new(
        //                         t.transform.translation.x,
        //                         t.transform.translation.y,
        //                         t.transform.translation.z,
        //                     );
        //                     let r = na::UnitQuaternion::from_quaternion(na::Quaternion {
        //                         coords: na::Vector4::new(
        //                             t.transform.rotation.x,
        //                             t.transform.rotation.y,
        //                             t.transform.rotation.z,
        //                             t.transform.rotation.w,
        //                         ),
        //                     });
        //                     let iso = na::Isometry3::from_parts(origin, r);

        //                     thread_tx2.send(Incoming::GroundTruthPose(iso)).unwrap();
        //                 }
        //             }
        //         },
        //     )
        //     .unwrap(),
        // );

        // let context = zmq::Context::new();
        // let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
        // self.rx_image = Some(rx);
        // let thread_tx = tx.clone();

        // spawn(move || {
        //     loop {
        //         println!("Connecting...");
        //         // let connection = connect(Url::parse("ws://localhost:3012/socket").unwrap());
        //         let subscriber = context.socket(zmq::SUB).unwrap();
        //         subscriber
        //             .connect(URL_IMAGE_PUB)
        //             .expect("failed connecting subscriber");
        //         subscriber.set_subscribe(b"").expect("failed subscribing");

        //         loop {
        //             // let envelope = subscriber
        //             //     .recv_string(0)
        //             //     .expect("failed receiving envelope")
        //             //     .unwrap();
        //             let message = subscriber.recv_bytes(0).expect("failed receiving message");
        //             godot_print!("image in {}", message.len());
        //             if message.len() > 0 {
        //                 thread_tx.send(message).unwrap();
        //             }
        //         }
        //     }
        // });
    }
    // This function will be called in every frame
    #[export]
    unsafe fn _process(&mut self, _owner: &Node, _delta: f64) {
        // if let Some(rx) = &self.rx_image {
        //     while let Ok(pixels) = rx.try_recv() {
        //         godot_print!("got image");
        //         let thumb = _owner
        //             .get_node("GUI/Cam/Thumb")
        //             .unwrap()
        //             .assume_safe()
        //             .cast::<Sprite>()
        //             .unwrap();
        //         // let texture = thumb.texture().unwrap().cast::<ImageTexture>().unwrap();
        //         let im = Image::new();
        //         im.load_jpg_from_buffer(TypedArray::from_vec(pixels));
        //         // im.create_from_data(1280, 960, true, Image::FORMAT_RGB8, TypedArray::from_vec(pixels));
        //         let imt = ImageTexture::new();

        //         imt.create_from_image(im, 7);
        //         (*thumb).set_texture(imt);
        //     }
        // }

        let mut edges = None;
        if let Some(rx) = &self.rx {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    Incoming::GroundTruthPose(iso) => {
                        godot_print!("TRANSFORM {:?}", iso);
                        if self.values.first_ground_truth_pose.is_none() {
                            self.values.first_ground_truth_pose = Some(iso);
                        } else if let Some(first_iso) = self.values.first_ground_truth_pose {
                            let iso = first_iso.inverse() * iso;
                            let ROT_ROS2GODOT: na::UnitQuaternion<f64> =
                                na::UnitQuaternion::from_euler_angles(
                                    -std::f64::consts::FRAC_PI_2,
                                    -std::f64::consts::FRAC_PI_2,
                                    0.,
                                );
                            let ROS2GODOT: na::Isometry3<f64> = na::Isometry3::from_parts(
                                na::Translation3::identity(),
                                ROT_ROS2GODOT,
                            );
                            let iso = ROS2GODOT * iso;
                            let marker = _owner
                                .get_node("Spatial/RosFrame/GTPose")
                                .unwrap()
                                .assume_safe()
                                .cast::<Spatial>()
                                .unwrap();
                            marker.set_transform(iso3_to_gd(&iso));

                            if let Some(camera_pose) = &self.values.camera_pose {
                                let scale = iso.translation.vector.magnitude()
                                    / camera_pose.translation.vector.magnitude();
                                godot_print!("SCALE={}", scale);
                                if scale.is_normal() {
                                    let frames = _owner
                                        .get_node("Spatial/Frames")
                                        .unwrap()
                                        .assume_safe()
                                        .cast::<Spatial>()
                                        .unwrap();

                                    frames.set_scale(Vector3::new(
                                        scale as f32,
                                        scale as f32,
                                        scale as f32,
                                    ));
                                }
                            }
                        }
                    }
                    Incoming::OpenVSlamPB(msg) => {
                        let last_image = ::base64::decode(msg.last_image).unwrap();
                        let thumb = _owner
                            .get_node("GUI/Cam/Thumb")
                            .unwrap()
                            .assume_safe()
                            .cast::<Sprite>()
                            .unwrap();
                        // let texture = thumb.texture().unwrap().cast::<ImageTexture>().unwrap();
                        let im = Image::new();
                        im.load_jpg_from_buffer(TypedArray::from_vec(last_image));
                        // im.create_from_data(1280, 960, true, Image::FORMAT_RGB8, TypedArray::from_vec(pixels));
                        let imt = ImageTexture::new();

                        imt.create_from_image(im, 7);
                        (*thumb).set_texture(imt);

                        let colormap: ListedColorMap = ListedColorMap::plasma();
                        for landmark in msg.landmarks.iter() {
                            if landmark.coords.len() != 0 {
                                if landmark.num_observations > self.values.max_lm_obs as i32 {
                                    godot_print!("lm max num ob {}", landmark.num_observations);
                                    self.values.max_lm_obs = landmark.num_observations as u32;
                                }
                                let val =
                                    0.5 + f64::min(0.5, landmark.num_observations as f64 / 24.0); //self.values.max_lm_obs as f64;
                                let color = colormap.vals
                                    [(val * (colormap.vals.len() - 1) as f64) as usize];
                                let color =
                                    Color::rgb(color[0] as f32, color[1] as f32, color[2] as f32);
                                self.values.landmarks.insert(
                                    landmark.id,
                                    (
                                        Vector3::new(
                                            landmark.coords[0] as f32,
                                            landmark.coords[1] as f32,
                                            landmark.coords[2] as f32,
                                        ),
                                        color,
                                        landmark.num_observations as u32,
                                    ),
                                );
                            } else {
                                self.values.landmarks.remove(&landmark.id);
                            }
                            // println!("landmark {:?}", landmark.color);
                            // vectors.push(landmark.coords);
                        }

                        for message in msg.messages.iter() {
                            let text = format!("[{}]: {}", message.tag, message.txt);
                            if message.tag == "TRACKING_STATE" {
                                let tracker_state = match message.txt.as_str() {
                                    "NotInitialized" => Some(TrackerState::NotInitialized),
                                    "Initializing" => Some(TrackerState::Initializing),
                                    "Tracking" => Some(TrackerState::Tracking),
                                    "Lost" => Some(TrackerState::Lost),
                                    _ => None,
                                };
                                if let Some(tracker_state) = tracker_state {
                                    self.values.tracker_state = tracker_state;
                                    self.navigator
                                        .send_tracker_state
                                        .send(tracker_state)
                                        .unwrap();
                                }
                            }
                            _owner.emit_signal("message", &[Variant::from_str(text)]);
                        }

                        for keyframe in msg.keyframes.iter() {
                            if let Some(pose) = &keyframe.pose {
                                let (vectors, _, _) = mat44_to_vertices(pose);

                                self.values.keyframes.insert(keyframe.id, vectors);

                                if self.values.marked_keyframe == None {
                                    self.values.marked_keyframe = Some(keyframe.id);
                                }
                            } else {
                                self.values.keyframes.remove(&keyframe.id);
                            }
                        }

                        if let Some(current_frame) = msg.current_frame {
                            let (vertices, rotation, translation) =
                                mat44_to_vertices(&current_frame);
                            self.values.current_frame = Some(vertices);
                            let cam_pose = mat44_to_isometry3(&current_frame);
                            self.values.camera_pose = Some(cam_pose);
                            let _ = self.navigator.send_cam_pose.send(cam_pose);
                        }

                        edges = Some(msg.edges);

                        // TODO get these working

                        // fn get_node<T: SubClass<gdnative::prelude::Node>>(owner: &Node, path: &str) -> TRef<T> {
                        //     owner
                        //         .get_node(path)
                        //         .unwrap()
                        //         .assume_safe()
                        //         .cast::<T>()
                        //         .unwrap()
                        // }

                        // fn draw_mesh(node: TRef<ImmediateGeometry>, vertices: Values<u32, Vector3D>, primitive: i64, color: Colors::C) {
                        //     node.clear();
                        //     node.begin(primitive, Null::null());
                        //     node.set_color(color.as_godot());
                        //     for v in vertices {
                        //         node.add_vertex(*v);
                        //     }
                        //     node.end();
                        // }

                        let frames_mesh = _owner
                            .get_node("Spatial/Frames/Frames")
                            .unwrap()
                            .assume_safe()
                            .cast::<ImmediateGeometry>()
                            .unwrap();
                        frames_mesh.clear();
                        for vertices in self.values.keyframes.values() {
                            frames_mesh.begin(Mesh::PRIMITIVE_LINE_STRIP, Null::null());
                            frames_mesh.set_color(Colors::FRAME.as_godot());
                            for v in vertices {
                                frames_mesh.add_vertex(*v);
                            }
                            frames_mesh.end();
                        }

                        let landmark_mesh = _owner
                            .get_node("Spatial/Frames/Landmarks")
                            .unwrap()
                            .assume_safe()
                            .cast::<ImmediateGeometry>()
                            .unwrap();
                        landmark_mesh.clear();
                        landmark_mesh.begin(Mesh::PRIMITIVE_POINTS, Null::null());
                        // landmark_mesh.set_color(Colors::LANDMARK1.as_godot());
                        for (v, c, num_obs) in self.values.landmarks.values() {
                            if num_obs >= &self.values.min_lm_obs {
                                landmark_mesh.set_color(*c);
                                landmark_mesh.add_vertex(*v);
                            }
                        }
                        landmark_mesh.end();

                        if let Some(edges_) = edges {
                            let edges_mesh = _owner
                                .get_node("Spatial/Frames/Edges")
                                .unwrap()
                                .assume_safe()
                                .cast::<ImmediateGeometry>()
                                .unwrap();
                            edges_mesh.clear();
                            edges_mesh.begin(Mesh::PRIMITIVE_LINES, Null::null());
                            edges_mesh.set_color(Colors::EDGE.as_godot());
                            for e in edges_.iter() {
                                let k0 = self.values.keyframes.get(&e.id0);
                                let k1 = self.values.keyframes.get(&e.id1);
                                if let (Some(k0), Some(k1)) = (k0, k1) {
                                    edges_mesh.add_vertex(k0[0]);
                                    edges_mesh.add_vertex(k1[0]);
                                }
                            }
                            edges_mesh.end();
                        }

                        // TODO convert this to some mesh
                        // if let Some(current_frame) = &self.values.current_frame {
                        //     _owner.emit_signal("current_frame", &[current_frame.to_variant()]);
                        // }

                        if let Some(camera_pose) = &self.values.camera_pose {
                            let transform = iso3_to_gd(camera_pose);
                            let marker = _owner
                                .get_node("Spatial/Frames/CurrentPose")
                                .unwrap()
                                .assume_safe()
                                .cast::<Spatial>()
                                .unwrap();
                            marker.set_transform(transform);

                            let marker = _owner
                                .get_node("Spatial/Frames/CamTarget")
                                .unwrap()
                                .assume_safe()
                                .cast::<Spatial>()
                                .unwrap();
                            marker.set_transform(transform);

                            let marker = _owner
                                .get_node("Spatial/Frames/BasePose")
                                .unwrap()
                                .assume_safe()
                                .cast::<CSGBox>()
                                .unwrap();
                            let base_pose = navigator::RobotBody::base_pose(*camera_pose, self.values.slam_scale);
                            marker.set_transform(iso3_to_gd(&base_pose));
                        }

                        // if let Some(ground_truth_pose) = self.values.ground_truth_pose {
                        //     let marker = _owner
                        //         .get_node("Spatial/Frames/GTPose")
                        //         .unwrap()
                        //         .assume_safe()
                        //         .cast::<Spatial>()
                        //         .unwrap();

                        //     marker.set_transform(ground_truth_pose);
                        // }

                        // if let Some(marked_keyframe) = &self.values.marked_keyframe {
                        //     if let Some(vertices) = &self.values.keyframes.get(marked_keyframe) {
                        //         let marker = _owner
                        //             .get_node("Spatial/Marker")
                        //             .unwrap()
                        //             .assume_safe()
                        //             .cast::<CSGBox>()
                        //             .unwrap();
                        //         marker.set_translation(vertices[0]);
                        //     }
                        // }
                    }
                }
            }
        }

        // let speed = 0.3;
        // let turn_speed = 0.5;

        let get_label = |path| {
            _owner
                .get_node(path)
                .unwrap()
                .assume_safe()
                .cast::<Label>()
                .unwrap()
        };

        // let go = |left, right, time| {
        //     let mut cmd = format!("{},{}", left, right);
        //     if let Some(time) = time {
        //         cmd = format!("{},{}", cmd, time);
        //     }
        //     self.pub_vel.send(&cmd, 0).expect("failed to send cmd");

        //     get_label("GUI/Speed").set_text(format!("{}", cmd));
        //     godot_print!("GO {}", cmd);
        //     // get_label("GUI/SpeedRight").set_text(format!(">{}", right));
        // };

        // if self.values.follow_target {
        //     if let Some(pose) = &self.values.camera_pose {
        //         if self.values.last_step_mark.should_move(&pose) {
        //             let speed_go = 0.3;
        //             let speed_turn = 0.3;
        //             let step_time = 0;

        //             let dx = self.values.target.0 - pose.translation.vector.x;
        //             let dz = self.values.target.2 - pose.translation.vector.z;
        //             let yaw_target = dx.atan2(dz);
        //             let yaw_bot = pose.rotation.euler_angles().1;
        //             let yawd = angle_difference(yaw_bot, yaw_target);
        //             let distance = dx.hypot(dz);
        //             godot_print!(
        //                 "from {:?} to {:?} is {}mm; yaw_target={} yaw_bot={} yawd={}",
        //                 (dx, dz),
        //                 self.values.target,
        //                 distance,
        //                 yaw_target,
        //                 yaw_bot,
        //                 yawd
        //             );

        //             if distance < 0.2 {
        //                 go(0., 0., None);
        //             } else if yawd.abs() < 0.3 {
        //                 go(speed_go, speed_go, Some(step_time));
        //             } else if yawd > 0. {
        //                 go(speed_turn, -speed_turn, Some(step_time));
        //             } else {
        //                 go(-speed_turn, speed_turn, Some(step_time));
        //             }

        //             self.values.last_step_mark = StepMark {
        //                 time: time::SystemTime::now(),
        //                 pose: pose.clone(),
        //             };
        //         }
        //     }
        // } else {
        //     if let Some(step) = self.values.step {
        //         godot_print!("STEP {:?}", step);
        //         go(step.0, step.1, Some(step.2 as i32));
        //         self.values.step = None;
        //         self.values.speed = None;
        //     } else if let Some(speed) = self.values.speed {
        //         go(speed.0, speed.1, None);
        //     }
        // }

        get_label("GUI/TrackerState").set_text(format!("{:?}", self.values.tracker_state));
        // let labelNode: Label = _owner.get_node(gd::NodePath::from_str("/root/GUI/SpeedRight")).unwrap().cast::<Label>();
        // labelNode.set_text(format!("{}", SpeedRight));
        // godot_print!("{}", cmd);
    }
}
