use gdnative::api::*;
use gdnative::prelude::*;

use std::sync::Arc;
use tokio;
use tokio::sync::{broadcast::error::RecvError, RwLock};

use crate::components::context::Context;
use crate::components::traits::Updatable;
use common::{msg::Msg, types::GDInput};

pub struct Teleop {
    joy_state: Arc<RwLock<JoyState>>, // frame: Receiver<Option<Vec<u8>>>,
                                      // save_btn_path: String
}

enum ControlMode {
    Joystick,
    Keyboard,
}
struct JoyState {
    control_mode: ControlMode,

    left: f64,
    right: f64,
    left_reversed: bool,
    right_reversed: bool,

    key_up: bool,
    key_right: bool,
    key_down: bool,
    key_left: bool,
    keyboard_speed: f64,

    weeder_speed: f64,
}
impl JoyState {
    fn new() -> Self {
        JoyState {
            control_mode: ControlMode::Joystick,

            left: 0.0,
            right: 0.0,
            left_reversed: false,
            right_reversed: false,

            key_up: false,
            key_right: false,
            key_down: false,
            key_left: false,
            keyboard_speed: 0.5,

            weeder_speed: 0.0,
        }
    }
    fn left_right(&self) -> (f64, f64) {
        match self.control_mode {
            ControlMode::Joystick => {
                let left = if self.left_reversed {
                    -self.left
                } else {
                    self.left
                };
                let right = if self.right_reversed {
                    -self.right
                } else {
                    self.right
                };
                (left, right)
            }
            ControlMode::Keyboard => {
                let mut left: f64 = 0.0;
                let mut right: f64 = 0.0;
                let speed = self.keyboard_speed;
                if self.key_up {
                    left += speed;
                    right += speed;
                }
                if self.key_down {
                    left -= speed;
                    right -= speed;
                }
                if self.key_left {
                    left -= speed;
                    right += speed;
                }
                if self.key_right {
                    left += speed;
                    right -= speed;
                }
                (left.max(-speed).min(speed), right.max(-speed).min(speed))
            }
        }
    }
}

const MAX_SPEED: f64 = 1.0;

impl Teleop {
    pub fn new(owner: TRef<Node>, path: String, context: &mut Context) -> Self {
        let joy_state = Arc::new(RwLock::new(JoyState::new()));

        // convert user input to speed commands and broadcast them
        {
            let publisher = context.broadcaster.publisher();
            let mut input_receiver = context.subscribe_input();
            let joy_state = Arc::clone(&joy_state);
            context.runtime().spawn(async move {
                loop {
                    match input_receiver.recv().await {
                        Ok(inp) => {
                            if let Some(inp) = inp {
                                let mut joy_state = joy_state.write().await;
                                match inp {
                                    GDInput::JoyMotion(event) => {
                                        joy_state.control_mode = ControlMode::Joystick;
                                        if event.axis == GlobalConstants::JOY_AXIS_6 {
                                            joy_state.left = event.axis_value.min(MAX_SPEED);
                                        } else if event.axis == GlobalConstants::JOY_AXIS_7 {
                                            joy_state.right = event.axis_value.min(MAX_SPEED);
                                        }
                                    }
                                    GDInput::JoyButton(event) => {
                                        joy_state.control_mode = ControlMode::Joystick;
                                        match event.button_index {
                                            GlobalConstants::JOY_BUTTON_4 => {
                                                joy_state.left_reversed = event.pressed
                                            }
                                            GlobalConstants::JOY_BUTTON_5 => {
                                                joy_state.right_reversed = event.pressed
                                            }
                                            GlobalConstants::JOY_BUTTON_1 => {
                                                joy_state.weeder_speed =
                                                    if event.pressed { 1.0 } else { 0.0 }
                                            }
                                            GlobalConstants::JOY_BUTTON_12
                                            | GlobalConstants::JOY_BUTTON_13
                                            | GlobalConstants::JOY_BUTTON_14
                                            | GlobalConstants::JOY_BUTTON_15 => {
                                                joy_state.control_mode = ControlMode::Keyboard;
                                                match event.button_index {
                                                    GlobalConstants::JOY_BUTTON_12 => {
                                                        joy_state.key_up = event.pressed
                                                    }
                                                    GlobalConstants::JOY_BUTTON_15 => {
                                                        joy_state.key_right = event.pressed
                                                    }
                                                    GlobalConstants::JOY_BUTTON_13 => {
                                                        joy_state.key_down = event.pressed
                                                    }
                                                    GlobalConstants::JOY_BUTTON_14 => {
                                                        joy_state.key_left = event.pressed
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    GDInput::Key(event) => {
                                        joy_state.control_mode = ControlMode::Keyboard;
                                        if !event.echo {
                                            match event.scancode {
                                                GlobalConstants::KEY_UP => {
                                                    joy_state.key_up = event.pressed
                                                }
                                                GlobalConstants::KEY_RIGHT => {
                                                    joy_state.key_right = event.pressed
                                                }
                                                GlobalConstants::KEY_DOWN => {
                                                    joy_state.key_down = event.pressed
                                                }
                                                GlobalConstants::KEY_LEFT => {
                                                    joy_state.key_left = event.pressed
                                                }
                                                _ => (),
                                            }
                                        }
                                    }
                                }
                                let (left, right) = joy_state.left_right();
                                publisher.send(Msg::Teleop((left, right))).ok();
                                publisher
                                    .send(Msg::SetWeederSpeed(joy_state.weeder_speed))
                                    .ok();
                            }
                        }
                        Err(RecvError::Lagged(lagged)) => {
                            println!("Teleop lagged {} input messages", lagged)
                        }
                        Err(RecvError::Closed) => {
                            break;
                        }
                    }
                }
            });
        }

        // HACK! periodically resend latest speed message because the robot stops if the latest command is too old
        {
            let publisher = context.broadcaster.publisher();
            let joy_state = Arc::clone(&joy_state);
            context.runtime().spawn(async move {
                loop {
                    let (left, right) = joy_state.read().await.left_right();
                    publisher.send(Msg::Teleop((left, right))).ok();
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                }
            });
        }

        Teleop {
            joy_state,
            // frame,
            // save_btn_path,
        }
    }
}

impl Updatable for Teleop {
    fn update(&self, _owner: &Node) {}
}
