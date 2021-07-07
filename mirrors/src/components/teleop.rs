use gdnative::api::*;
use gdnative::prelude::*;

use tokio;
use tokio::sync::{RwLock, broadcast::error::RecvError};
use std::sync::Arc;

use crate::components::traits::Updatable;
use crate::utils::get_node;
use crate::components::context::Context;
use common::types::GDInput;

pub struct Teleop {
    joy_state: Arc<RwLock<JoyState>>
    // frame: Receiver<Option<Vec<u8>>>,
    // save_btn_path: String,
}

struct JoyState {
    left: f64,
    right: f64,
    left_reversed: bool,
    right_reversed: bool,
}
impl JoyState {
    fn new() -> Self {
        JoyState {
            left: 0.0,
            right: 0.0,
            left_reversed: false,
            right_reversed: false,
        }
    }
    fn left_right(&self) -> (f64, f64) {
        let left = if self.left_reversed { -self.left } else { self.left };
        let right = if self.right_reversed { -self.right } else { self.right };
        (left, right)
    }
}

const MAX_SPEED: f64 = 1.0;

impl Teleop {
    pub fn new(owner: TRef<Node>, path: String, context: &mut Context) -> Self {
        let joy_state = Arc::new(RwLock::new(JoyState::new()));
        {
            let mut input_receiver = context.subscribe_input();
            let client = Arc::clone(&context.client);
            let joy_state = Arc::clone(&joy_state);
            context.runtime().spawn(async move {
                loop {
                    match input_receiver.recv().await {
                        Ok(inp) => {
                            if let Some(inp) = inp {
                                let mut joy_state = joy_state.write().await;
                                match inp {
                                    GDInput::JoyMotion(event) => {
                                        if event.axis == 6 {
                                            joy_state.left = event.axis_value.min(MAX_SPEED);
                                        }
                                        else if event.axis == 7 {
                                            joy_state.right = event.axis_value.min(MAX_SPEED);
                                        }
                                    }
                                    GDInput::JoyButton(event) => {
                                        if event.button_index == 4 {
                                            joy_state.left_reversed = event.pressed;
                                        }
                                        else if event.button_index == 5 {
                                            joy_state.right_reversed = event.pressed;
                                        }
                                    }
                                }
                                let client = client.read().await;
                                let (left, right) = joy_state.left_right();
                                client.set_speed(left, right).await;
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
        

        Teleop {
            joy_state,
            // frame,
            // save_btn_path,
        }
    }
}

impl Updatable for Teleop {
    fn update(&self, owner: &Node) {
        
    }
}