use std::sync::Arc;

use common::types::NavGoal;
use common::types::NavigationMode;
use common::types::NavigatorState;
use common::types::RobotParams;
use common::types::SystemStatus;
use gdnative::api::*;
use gdnative::prelude::*;
use tokio_stream::StreamExt;
use regex::Regex;

use crate::components;
use crate::ui_state::MirrorsState;

use common::msg::Msg;

/// The Game "class"
#[derive(NativeClass)]
#[inherit(Node)]
// #[register_with(Self::register_builder)]
#[register_with(Self::register_signals)]
pub struct Game {
    name: String,
    components: Vec<Box<dyn components::traits::Updatable>>,
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
            name: "robot_params",
            // Argument list used by the editor for GUI and generation of GDScript handlers. It can be omitted if the signal is only used from code.
            args: &[SignalArgument {
                name: "robot_params",
                default: RobotParams::default().to_variant(),
                export_info: ExportInfo::new(VariantType::GodotString),
                usage: PropertyUsage::DEFAULT,
            }],
        });

        builder.add_signal(Signal {
            name: "navigator_state",
            // Argument list used by the editor for GUI and generation of GDScript handlers. It can be omitted if the signal is only used from code.
            args: &[SignalArgument {
                name: "navigator_state",
                default: NavigatorState::default().to_variant(),
                export_info: ExportInfo::new(VariantType::GodotString),
                usage: PropertyUsage::DEFAULT,
            }],
        });

        builder.add_signal(Signal {
            name: "ui_state",
            // Argument list used by the editor for GUI and generation of GDScript handlers. It can be omitted if the signal is only used from code.
            args: &[SignalArgument {
                name: "ui_state",
                default: MirrorsState::default().to_variant(),
                export_info: ExportInfo::new(VariantType::GodotString),
                usage: PropertyUsage::DEFAULT,
            }],
        });


        builder.add_signal(Signal {
            name: "system_status",
            // Argument list used by the editor for GUI and generation of GDScript handlers. It can be omitted if the signal is only used from code.
            args: &[SignalArgument {
                name: "system_status",
                default: SystemStatus::default().to_variant(),
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

    fn send_to_robot(&mut self, msg: Msg) {
        self.context
            .broadcaster
            .publisher()
            .send(msg)
            .ok();
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
    fn reconnect(&mut self, _owner: TRef<Node>, robot_address: String) {
        let client = Arc::clone(&self.context.client);
        self.context.ui_state.add_robot_address(robot_address.clone());
        self.context.rt.block_on(async {
            client.write().await.reconnect(robot_address).await.ok();
        });
    }

    #[export]
    fn set_viz_scale(&mut self, _owner: TRef<Node>, viz_scale: f64) {
        let client = Arc::clone(&self.context.client);
        self.context.ui_state.set_viz_scale(viz_scale);
        // self.context.rt.block_on(async {        
        //     client.write().await.reconnect(robot_address).await.ok();
        // });
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
                    let path = dirs::picture_dir().unwrap().join(folder);
                    std::fs::create_dir_all(path.clone()).expect("Failed to create directory for images");
                    let paths = std::fs::read_dir(path.clone()).unwrap();
                    let re = Regex::new(r"_(\d+)\.jpe?g").unwrap();
                    let mut max_index = 0;
                    for path in paths {
                        // println!("Name: {:?}", path.unwrap().path().file_name());
                        if let Some(cap) = re.captures(&path.unwrap().path().file_name().unwrap().to_str().unwrap()) {
                            let idx = cap[1].parse::<u32>().unwrap();
                            if idx > max_index {
                                max_index = idx;
                            }
                        }
                    }
                    let path = path.join(format!("image_{}.jpg", max_index+1));
                    println!("save image {:?}", path);
                    std::fs::write(path, frame.jpeg).expect("Failed to save image");
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
                    println!("didn't receive image to save");
                }
            }
        });
        
    }

    #[export]
    fn save_map(&mut self, _owner: TRef<Node>, map_name: String) {
        self.send_to_robot(Msg::SaveMapDB(map_name));
    }

    #[export]
    fn select_map(&mut self, _owner: TRef<Node>, map_name: String) {
        let map_name = if map_name.is_empty() { None } else { Some(map_name) };
        self.send_to_robot(Msg::SelectMap(map_name));
    }

    #[export]
    fn set_navigation_mode(&mut self, _owner: TRef<Node>, nav_mode: usize) {
        let nav_mode = [NavigationMode::Teleop, NavigationMode::Goal, NavigationMode::Waypoints][nav_mode];
        self.send_to_robot(Msg::SetNavigationMode(nav_mode));
    }

    #[export]
    fn select_target(&mut self, _owner: TRef<Node>, x: f64, z: f64) {
        let scale = self.context.ui_state.state.read().unwrap().map_to_viz_scale();
        let mut goal = NavGoal::new(x, z);
        goal.div(scale);
        self.send_to_robot(Msg::NavTarget(goal));
    }

    #[export]
    fn set_waypoints(&mut self, _owner: TRef<Node>, mut waypoints: Vec<NavGoal>) {
        let scale = self.context.ui_state.state.read().unwrap().map_to_viz_scale();
        for nav_goal in &mut waypoints {
            nav_goal.div(scale);
        }

        self.send_to_robot(Msg::Waypoints(waypoints));
    }

    #[export]
    fn enable_auto_nav(&mut self, _owner: TRef<Node>, enable: bool) {
        self.send_to_robot(Msg::EnableAutoNav(enable));
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
        // self.components.push(Box::new(components::MapHandler::new(
        //     owner,
        //     "GUI/VBox".into(),
        //     &mut self.context,
        // )));
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
