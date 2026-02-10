use eframe::wgpu;
use vec_utils::angle::AngleDegrees;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::ANGLE_EPSILON_DEGREES;
use crate::car::Car;

struct Pressed {
    rotate_up: bool,
    rotate_down: bool,
    reset: bool
}

pub struct InputHandler {
    pressed: Pressed
}

impl Pressed {
    fn new() -> Self {
        Self {
            rotate_up: false,
            rotate_down: false,
            reset: false
        }
    }
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            pressed: Pressed::new()
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state,
                        physical_key: PhysicalKey::Code(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    KeyCode::BracketLeft => {
                        self.pressed.rotate_up = is_pressed;
                        true
                    }
                    KeyCode::BracketRight => {
                        self.pressed.rotate_down = is_pressed;
                        true
                    }
                    KeyCode::KeyI => {
                        self.pressed.reset = is_pressed;
                        true
                    }
                    _ => false
                }
            }
            _ => false
        }
    }

    pub fn update_car(&self, moved: Option<&mut Car>) -> (bool, bool) {
        if self.pressed.reset || moved.is_none() {
            return (true, true);
        }
        let rotation = if self.pressed.rotate_down {
            -ANGLE_EPSILON_DEGREES
        } else {
            0.0
        } + if self.pressed.rotate_up {
            ANGLE_EPSILON_DEGREES
        } else {
            0.0
        };
        if rotation != 0.0 {
            moved.unwrap().rotate(AngleDegrees::new(rotation));
            return (true, false);
        }
        (false, false)
    }
}
