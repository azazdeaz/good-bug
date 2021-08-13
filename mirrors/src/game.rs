use std::sync::Arc;

use common::types::Point3;
use gdnative::api::*;
use gdnative::prelude::*;
use tokio_stream::StreamExt;

use crate::components;

use common::msg::Msg;

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
        godot_print!("build game");
        Game {
            name: "".to_string(),
            components: Vec::new(),
            context: components::context::Context::new(),
        }
    }

    #[export]
    fn _input(&self, owner: &Node, event: Ref<InputEvent>) {
        use common::types::{GDInput, InJoyButton, InJoyMotion};
        let event = unsafe {
            let event = event.assume_safe();
            if let Some(event) = event.cast::<InputEventJoypadMotion>() {
                Some(GDInput::JoyMotion(InJoyMotion {
                    axis: event.axis(),
                    axis_value: event.axis_value(),
                }))
            } else if let Some(event) = event.cast::<InputEventJoypadButton>() {
                Some(GDInput::JoyButton(InJoyButton {
                    button_index: event.button_index(),
                    pressed: event.is_pressed(),
                }))
            } else {
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
        self.context
            .signal_map
            .callback(id, SignalData::FFF(f1, f2, f3));
    }

    #[export]
    fn signal_map_callback_b(&mut self, _owner: TRef<Node>, b: bool, id: u32) {
        self.context.signal_map.callback(id, SignalData::B(b));
    }

    #[export]
    fn enable_raw_preview(&mut self, _owner: TRef<Node>, enable: bool) {
        self.context
            .broadcaster
            .publisher()
            .send(Msg::UseRawPreview(enable))
            .ok();
    }

    #[export]
    fn restart_slam(&mut self, _owner: TRef<Node>) {
        self.context
            .broadcaster
            .publisher()
            .send(Msg::TerminateSlam)
            .ok();
    }

    #[export]
    fn reconnect(&mut self, _owner: TRef<Node>) {
        let client = Arc::clone(&self.context.client);
        self.context.rt.block_on(async {
            client.write().await.reconnect().await;
        });
    }

    #[export]
    fn save_image(&mut self, _owner: TRef<Node>, folder: String, filename: String) {
        let mut stream = self
            .context
            .broadcaster
            .stream()
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .filter(|m| matches!(m, Msg::Frame(_)));
        self.context.runtime().spawn(async move {
            
            tokio::select! {
                Some(Msg::Frame(frame)) = stream.next() => {
                    let path = std::path::Path::new("/tmp").join(folder);
                    std::fs::create_dir_all(path.clone()).expect("Failed to create directory for images");
                    let path = path.join(filename);
                    println!("save image {:?}", path);
                    std::fs::write(path, frame).expect("Failed to save image");
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
                    println!("didn't receive image to save");
                }
            }
        });
        
    }

    #[export]
    fn select_target(&mut self, _owner: TRef<Node>, x: f64, y: f64, z: f64) {
        let scale = self.context.ui_state.state.read().unwrap().map_to_viz_scale();
        let point = Point3::new(x / scale, y / scale, z / scale);
        self.context
            .broadcaster
            .publisher()
            .send(Msg::NavTarget(point))
            .ok();
    }

    #[export]
    unsafe fn _ready(&mut self, owner: TRef<Node>) {
        self.name = "Game".to_string();

        
        self.components.push(Box::new(components::CameraPose::new(
            owner,
            "GUI/ViewportContainer/Viewport/Spatial".into(),
            &mut self.context,
        )));
        self.components.push(Box::new(components::Landmarks::new(
            owner,
            "GUI/ViewportContainer/Viewport/Spatial".into(),
            &mut self.context,
        )));
        self.components.push(Box::new(components::Keyframes::new(
            owner,
            "GUI/ViewportContainer/Viewport/Spatial".into(),
            &mut self.context,
        )));
        self.components.push(Box::new(components::Edges::new(
            owner,
            "GUI/ViewportContainer/Viewport/Spatial".into(),
            &mut self.context,
        )));
        self.components.push(Box::new(components::GroundPlane::new(
            owner,
            "GUI/ViewportContainer/Viewport/Spatial".into(),
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
