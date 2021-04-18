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
    pub_vel: zmq::Socket,
    rx: Option<Receiver<items::VSlamMap>>,
    rx_image: Option<Receiver<Vec<u8>>>,
}

use prost::Message;
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/map_segment.rs"));
}
use std::collections::HashMap;

const URL_IMAGE_PUB: &str = "tcp://192.168.50.234:5560";

#[derive(PartialEq, Clone)]
struct Pose {
    rotation: Array2<f64>,
    translation: Array2<f64>,
}
struct StepMark {
    time: time::SystemTime,
    pose: Pose,
}
impl StepMark {
    fn should_move(&self, pose: &Pose) -> bool {
        pose != &self.pose || self.time.elapsed().unwrap().as_secs() > 1
    }
}
struct Values {
    landmarks: HashMap<u32, Vector3>,
    keyframes: HashMap<u32, Vec<Vector3>>,
    current_frame: Option<Vec<Vector3>>,
    last_step_mark: StepMark,
    target: (f64, f64, f64),
    follow_target: bool,
    speed: Option<(f64, f64)>,
    step: Option<(f64, f64, f64)>,
    tracker_state: TrackerState,
    marked_keyframe: Option<u32>,
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

extern crate nalgebra as na;

use ndarray::prelude::*;
use ndarray::{stack, Array, Axis, OwnedRepr};

#[derive(Debug)]
enum TrackerState {
    NotInitialized,
    Initializing,
    Tracking,
    Lost,
}



mod Colors {
    use gdnative::prelude::Color;

    pub struct C {
        r: i32,
        g: i32,
        b: i32
    }
    impl C {
        pub fn as_godot(&self) -> Color {
            Color::rgb(self.r as f32 / 255., self.g as f32 / 255., self.b as f32 / 255.)
        }
    }

    pub const FRAME: C = C { r: 0xe7, g: 0x83, b: 0xfc };
    pub const EDGE: C = C { r: 0x63, g: 0x92, b: 0xff };
    pub const CURRENT_FRAME: C = C { r: 0xff, g: 0x77, b: 0x5e };
    pub const LANDMARK1: C = C { r: 0x1c, g: 0xff, b: 0x9f };
    pub const LANDMARK2: C = C { r: 0x96, g: 0xff, b: 0x08 };
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

const W: Wireframes = Wireframes {_zero_keyframe: None };

fn mat44_to_vertices (pose: &items::v_slam_map::Mat44) -> (Vec<Vector3>, Array2<f64>, Array2<f64>) {
    let pose = pose.pose.to_vec();
    let pose = Array::from_vec(pose).into_shape((4, 4)).unwrap();
    let pose = inv_pose(pose);
    let rotation = pose.slice(s![0..3, 0..3]).to_owned();
    let translation = pose.slice(s![0..3, 3..4]).to_owned();

    let vertices = W.zero_keyframe(&rotation, &translation);

    // let mut vectors: TypedArray<Vector3> = TypedArray::default();
    // for v in vertices.axis_iter(Axis(1)) {
    //     vectors.push(Vector3::new(v[0] as f32, v[1] as f32, v[2] as f32));
    // }
    // let mut vectors: Vec<Vector3> = vec![];
    // for v in vertices.axis_iter(Axis(1)) {
    //     vectors.push(Vector3::new(v[0] as f32, v[1] as f32, v[2] as f32));
    // }
    let vertices = vertices.axis_iter(Axis(1));
    let vertices: Vec<Vector3> = vertices.map(|v| Vector3::new(v[0] as f32, v[1] as f32, v[2] as f32)).collect();
    (vertices, rotation, translation)
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
            name: "tick",
            args: &[],
        });

        builder.add_signal(Signal {
            name: "dry_protobuf",
            // Argument list used by the editor for GUI and generation of GDScript handlers. It can be omitted if the signal is only used from code.
            args: &[SignalArgument {
                name: "data",
                default: Variant::from_str(""),
                export_info: ExportInfo::new(VariantType::ByteArray),
                usage: PropertyUsage::DEFAULT,
            }],
        });

        builder.add_signal(Signal {
            name: "tick_with_data",
            // Argument list used by the editor for GUI and generation of GDScript handlers. It can be omitted if the signal is only used from code.
            args: &[SignalArgument {
                name: "data",
                default: Variant::from_i64(100),
                export_info: ExportInfo::new(VariantType::I64),
                usage: PropertyUsage::DEFAULT,
            }],
        });

        builder.add_signal(Signal {
            name: "position",
            // Argument list used by the editor for GUI and generation of GDScript handlers. It can be omitted if the signal is only used from code.
            args: &[SignalArgument {
                name: "data",
                default: Variant::from_vector3(&Vector3::new(0., 0., 0.)),
                export_info: ExportInfo::new(VariantType::Vector3),
                usage: PropertyUsage::DEFAULT,
            }],
        });

        builder.add_signal(Signal {
            name: "points",
            // Argument list used by the editor for GUI and generation of GDScript handlers. It can be omitted if the signal is only used from code.
            args: &[SignalArgument {
                name: "data",
                default: Variant::from_vector3_array(&TypedArray::default()),
                export_info: ExportInfo::new(VariantType::Vector3Array),
                usage: PropertyUsage::DEFAULT,
            }],
        });

        builder.add_signal(Signal {
            name: "keyframe_vertices",
            // Argument list used by the editor for GUI and generation of GDScript handlers. It can be omitted if the signal is only used from code.
            args: &[SignalArgument {
                name: "data",
                default: Variant::from_array(&VariantArray::new_shared()),
                export_info: ExportInfo::new(VariantType::Vector3Array),
                usage: PropertyUsage::DEFAULT,
            }],
        });

        builder.add_signal(Signal {
            name: "edges",
            // Argument list used by the editor for GUI and generation of GDScript handlers. It can be omitted if the signal is only used from code.
            args: &[SignalArgument {
                name: "data",
                default: Variant::from_array(&VariantArray::new_shared()),
                export_info: ExportInfo::new(VariantType::Vector3Array),
                usage: PropertyUsage::DEFAULT,
            }],
        });

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
        godot_print!("Game is created!");

        let context = zmq::Context::new();
        let publisher = context.socket(zmq::PUB).unwrap();
        publisher
            .bind("tcp://*:5567")
            .expect("failed binding publisher");

        Game {
            name: "".to_string(),
            values: Values {
                landmarks: HashMap::new(),
                keyframes: HashMap::new(),
                current_frame: Some(vec![Vector3::new(0., 0., 0.)]),
                last_step_mark: StepMark {
                    time: time::SystemTime::now(),
                    pose: Pose {
                        rotation: Array2::eye(3),
                        translation: Array2::zeros((3, 1)),
                    },
                },
                target: (0., 0., 0.),
                follow_target: false,
                speed: None,
                step: Some((0., 0., 0.)),
                tracker_state: TrackerState::NotInitialized,
                marked_keyframe: None,
            },
            pub_vel: publisher,
            rx: None,
            rx_image: None,
        }
    }

    #[export]
    fn set_target(&mut self, _owner: TRef<Node>, x: f64, y: f64, z: f64) {
        self.values.speed = None;
        self.values.target = (x, y, z);
    }

    #[export]
    fn set_follow_target(&mut self, _owner: TRef<Node>, on: bool) {
        godot_print!("follow_target {}", on);
        self.values.follow_target = on;
    }

    #[export]
    fn set_speed(&mut self, _owner: TRef<Node>, left: f64, right: f64) {
        let left = (left * 100.).floor() / 100.;
        let right = (right * 100.).floor() / 100.;
        self.values.speed = Some((left, right));
    }

    #[export]
    fn set_step(&mut self, _owner: TRef<Node>, left: f64, right: f64, time: f64) {
        self.values.step = Some((left, right, time));
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

        let (tx, rx): (Sender<items::VSlamMap>, Receiver<items::VSlamMap>) = mpsc::channel();
        self.rx = Some(rx);
        let thread_tx = tx.clone();

        spawn(move || {
            loop {
                println!("Connecting...");
                // let connection = connect(Url::parse("ws://localhost:3012/socket").unwrap());
                let subscriber = context.socket(zmq::SUB).unwrap();
                subscriber
                    .connect("tcp://127.0.0.1:5566")
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
                    thread_tx.send(msg).unwrap();
                }
            }
        });

        let context = zmq::Context::new();
        let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
        self.rx_image = Some(rx);
        let thread_tx = tx.clone();

        spawn(move || {
            loop {
                println!("Connecting...");
                // let connection = connect(Url::parse("ws://localhost:3012/socket").unwrap());
                let subscriber = context.socket(zmq::SUB).unwrap();
                subscriber
                    .connect(URL_IMAGE_PUB)
                    .expect("failed connecting subscriber");
                subscriber.set_subscribe(b"").expect("failed subscribing");

                loop {
                    // let envelope = subscriber
                    //     .recv_string(0)
                    //     .expect("failed receiving envelope")
                    //     .unwrap();
                    let message = subscriber.recv_bytes(0).expect("failed receiving message");
                    godot_print!("image in {}", message.len());
                    if message.len() > 0 {
                        thread_tx.send(message).unwrap();
                    }
                }
            }
        });
    }
    // This function will be called in every frame
    #[export]
    unsafe fn _process(&mut self, _owner: &Node, delta: f64) {
        let mut new_pose: Option<Pose> = None;
        // let mut vectors: TypedArray<Vector3> = TypedArray::default();
        // for x in -20..=20 {
        //     for y in -20..=20 {
        //         for z in -20..=20 {
        //             vectors.push(Vector3::new(x as f32, y as f32, z as f32));
        //         }
        //     }
        // }
        // println!("processed");

        // _owner.emit_signal(
        //     "points",
        //     &[Variant::from_vector3_array(&vectors)],
        // );


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
            _owner.emit_signal("tick", &[]);

            while let Ok(msg) = rx.try_recv() {
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

                for landmark in msg.landmarks.iter() {
                    if landmark.coords.len() != 0 {
                        self.values.landmarks.insert(
                            landmark.id,
                            Vector3::new(
                                landmark.coords[0] as f32,
                                landmark.coords[1] as f32,
                                landmark.coords[2] as f32,
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
                            self.values.tracker_state = tracker_state
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
                    let (vertices, rotation, translation) = mat44_to_vertices(&current_frame);
                    self.values.current_frame = Some(vertices);
                    new_pose = Some(Pose {
                        rotation,
                        translation,
                    });
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
                landmark_mesh.set_color(Colors::LANDMARK1.as_godot());
                for v in self.values.landmarks.values() {
                    landmark_mesh.add_vertex(*v);
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
                if let Some(current_frame) = &self.values.current_frame {
                    _owner.emit_signal("current_frame", &[current_frame.to_variant()]);
                }

                if let Some(marked_keyframe) = &self.values.marked_keyframe {
                    if let Some(vertices) = &self.values.keyframes.get(marked_keyframe) {
                        let marker = _owner
                            .get_node("Spatial/Marker")
                            .unwrap()
                            .assume_safe()
                            .cast::<CSGBox>()
                            .unwrap();
                        marker.set_translation(vertices[0]);
                    }
                }
            }
        }

        

        let speed = 0.3;
        let turn_speed = 0.5;

        let get_label = |path| {
            _owner
                .get_node(path)
                .unwrap()
                .assume_safe()
                .cast::<Label>()
                .unwrap()
        };

        let go = |left, right, time| {
            let mut cmd = format!("{},{}", left, right);
            if let Some(time) = time {
                cmd = format!("{},{}", cmd, time);
            }
            self.pub_vel.send(&cmd, 0).expect("failed to send cmd");

            get_label("GUI/Speed").set_text(format!("{}", cmd));
            // get_label("GUI/SpeedRight").set_text(format!(">{}", right));
        };

        if self.values.follow_target {
            if let Some(pose) = new_pose {
                if self.values.last_step_mark.should_move(&pose) {
                    let speed_go = 0.4;
                    let speed_turn = 0.6;
                    let step_time = 0.12;

                    let dx = self.values.target.0 - pose.translation[[0, 0]];
                    let dz = self.values.target.2 - pose.translation[[2, 0]];
                    let yaw_target = dx.atan2(dz);
                    let m = na::Matrix3::from_row_slice(&pose.rotation.to_owned().into_raw_vec());
                    let yaw_bot = na::Rotation3::from_matrix(&m).euler_angles().1;
                    let yawd = angle_difference(yaw_bot, yaw_target);
                    let distance = dx.hypot(dz);
                    godot_print!(
                        "from {:?} to {:?} is {}mm; yaw_target={} yaw_bot={} yawd={}",
                        (dx, dz),
                        self.values.target,
                        distance,
                        yaw_target,
                        yaw_bot,
                        yawd
                    );

                    if distance < 0.2 {
                        go(0., 0., None);
                    } else if yawd.abs() < 0.3 {
                        go(speed_go, speed_go, Some(step_time));
                    } else if yawd > 0. {
                        go(speed_turn, -speed_turn, Some(step_time));
                    } else {
                        go(-speed_turn, speed_turn, Some(step_time));
                    }

                    self.values.last_step_mark = StepMark {
                        time: time::SystemTime::now(),
                        pose,
                    };
                }
            }
        } else {
            if let Some(step) = self.values.step {
                godot_print!("STEP {:?}", step);
                go(step.0, step.1, Some(step.2));
                self.values.step = None;
                self.values.speed = None;
            } else if let Some(speed) = self.values.speed {
                go(speed.0, speed.1, None);
            }
        }

        get_label("GUI/TrackerState").set_text(format!("{:?}", self.values.tracker_state));
        // let labelNode: Label = _owner.get_node(gd::NodePath::from_str("/root/GUI/SpeedRight")).unwrap().cast::<Label>();
        // labelNode.set_text(format!("{}", SpeedRight));
        // godot_print!("{}", cmd);
    }
}
