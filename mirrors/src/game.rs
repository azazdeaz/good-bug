use gdnative::api::*;
use gdnative::prelude::*;

use crate::components;

/// The Game "class"
#[derive(NativeClass)]
#[inherit(Node)]
// #[register_with(Self::register_builder)]
#[register_with(Self::register_signals)]
pub struct Game {
    name: String,
    components: Vec<Box<components::traits::Updatable>>,
    context: components::context::Context,
}

use crate::signal_map::SignalData;


extern crate yaml_rust;

extern crate nalgebra as na;



// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl Game {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
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
        let game = Game {
            name: "".to_string(),
            components: Vec::new(),
            context: components::context::Context::new(),
        };
        godot_print!("build game");
        game
    }

    #[export]
    fn _input(&self, owner: &Node, event: Ref<InputEvent>) {
        use common::types::{ InJoyMotion, InJoyButton, GDInput };
        let event = unsafe {
            let event = event.assume_safe();
            if let Some(event) = event.cast::<InputEventJoypadMotion>() {
                Some(GDInput::JoyMotion(InJoyMotion { 
                    axis: event.axis(),
                    axis_value: event.axis_value(),
                }))
            }
            else if let Some(event) = event.cast::<InputEventJoypadButton>() {
                Some(GDInput::JoyButton(InJoyButton { 
                    button_index: event.button_index(),
                    pressed: event.is_pressed(),
                }))
            }
            else {
                None
            }
        };
        if let Some(event) = event {
            self.context.send_input(event);
        }
    }

    #[export]
    fn signal_map_callback(&mut self, _owner: TRef<Node>, id: u32) {
        self.context.signal_map.callback(id, SignalData::Empty);
    }

    #[export]
    fn signal_map_callback_fff(&mut self, _owner: TRef<Node>, f1: f64, f2: f64, f3: f64, id: u32) {
        self.context.signal_map.callback(id, SignalData::FFF(f1, f2, f3));
    }

 


    // #[export]
    // fn estimate_scale(&mut self, owner: TRef<Node>) {
    //     let z = self
    //         .values
    //         .landmarks
    //         .values()
    //         .map(|v| v.0.y)
    //         .filter(|z| z.is_sign_positive());
    //     // .collect::<Vec<f32>>();

    //     let z = ndarray::Array::from_iter(z);
    //     let ground_level = z.mean();
    //     // let z = na::VectorN::from_vec(z);
    //     godot_print!("mean {:?}", ground_level);

    //     if let Some(ground_level) = ground_level {
    //         unsafe {
    //             let t = Transform::translate(Vector3::new(0.0, -ground_level, 0.0));
    //             owner
    //                 .find_node("Ground", true, true)
    //                 .unwrap()
    //                 .assume_safe()
    //                 .cast::<CSGMesh>()
    //                 .unwrap()
    //                 .set_translation(Vector3::new(0.0, -ground_level, 0.0));

    //             self.values.slam_scale =
    //                 ground_level as f64 / navigator::RobotBody::get_cam_height();
    //             self.navigator
    //                 .send_slam_scale
    //                 .send(self.values.slam_scale)
    //                 .unwrap();
    //         }
    //     }
    // }

    #[export]
    unsafe fn _ready(&mut self, owner: TRef<Node>) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        self.name = "Game".to_string();
        godot_print!("{} is ready!!!mak", self.name);
        self.components.push(Box::new(components::CameraPose::new(
            owner,
            "Spatial".into(),
            &mut self.context,
        )));
        self.components.push(Box::new(components::Landmarks::new(
            owner,
            "Spatial".into(),
            &mut self.context,
        )));
        self.components.push(Box::new(components::Keyframes::new(
            owner,
            "Spatial".into(),
            &mut self.context,
        )));
        self.components.push(Box::new(components::Edges::new(
            owner,
            "Spatial".into(),
            &mut self.context,
        )));
        self.components.push(Box::new(components::Frame::new(
            owner,
            "GUI/VBox".into(),
            &mut self.context,
        )));
        self.components.push(Box::new(components::Status::new(
            owner,
            "GUI/VBox".into(),
            &mut self.context,
        )));
        self.components.push(Box::new(components::MapHandler::new(
            owner,
            "GUI/VBox".into(),
            &mut self.context,
        )));
        self.components.push(Box::new(components::Teleop::new(
            owner,
            "GUI/VBox".into(),
            &mut self.context,
        )));
    }

    #[export]
    unsafe fn _process(&mut self, owner: &Node, _delta: f64) {
        for component in &self.components {
            component.update(owner);
        }
    }
}
