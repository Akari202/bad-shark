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
                    _ => { 
                        false
                    }
                }
            }
            _ => {
                false
            },
        }
    }
    
    pub fn update_car(&self, ride: &Car, mut moved: &mut Car) -> (bool, bool) {
        let mut change = (false, false);
        if self.pressed.rotate_down {
            moved.rotate(AngleDegrees::new(-1.0 * ANGLE_EPSILON_DEGREES));
            change.0 = true;
        }
        if self.pressed.rotate_up {
            moved.rotate(AngleDegrees::new(ANGLE_EPSILON_DEGREES));
            change.0 = true;
        }
        if self.pressed.reset {
            change = (true, true);
        }
        change
    }
}