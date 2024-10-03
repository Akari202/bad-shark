use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::OPENGL_TO_WGPU_MATRIX;

pub(crate) struct Camera {
    pub(crate) eye: cgmath::Point3<f32>,
    pub(crate) target: cgmath::Point3<f32>,
    pub(crate) up: cgmath::Vector3<f32>,
    pub(crate) aspect: f32,
    pub(crate) fovy: f32,
    pub(crate) znear: f32,
    pub(crate) zfar: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // 1.
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        // 2.
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        // 3.
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct CameraUniform {
    // We can't use cgmath with bytemuck directly, so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub(crate) fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub(crate) fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

struct Pressed {
    pub zoom_in: bool,
    pub zoom_out: bool,
    pub orbit_right: bool,
    pub orbit_left: bool,
    pub orbit_down: bool,
    pub orbit_up: bool,
    pub pan_left: bool,
    pub pan_right: bool,
    pub pan_down: bool,
    pub pan_up: bool
}

impl Pressed {
    pub fn new() -> Self {
        Self {
            zoom_in: false,
            zoom_out: false,
            orbit_right: false,
            orbit_left: false,
            orbit_down: false,
            orbit_up: false,
            pan_left: false,
            pan_right: false,
            pan_down: false,
            pan_up: false
        }
    }
}

pub(crate) struct CameraController {
    speed: f32,
    pressed: Pressed
}

impl CameraController {
    pub(crate) fn new(speed: f32) -> Self {
        Self {
            speed,
            pressed: Pressed::new()
        }
    }

    pub(crate) fn process_events(&mut self, event: &WindowEvent) -> bool {
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
                    KeyCode::Equal => {
                        self.pressed.zoom_in = is_pressed;
                        true
                    }
                    KeyCode::Minus => {
                        self.pressed.zoom_out = is_pressed;
                        true
                    }
                    KeyCode::KeyA => {
                        self.pressed.orbit_left = is_pressed;
                        true
                    }
                    KeyCode::KeyD => {
                        self.pressed.orbit_right = is_pressed;
                        true
                    }
                    KeyCode::KeyW => {
                        self.pressed.orbit_up = is_pressed;
                        true
                    }
                    KeyCode::KeyS => {
                        self.pressed.orbit_down = is_pressed;
                        true
                    }
                    KeyCode::ArrowUp => {
                        self.pressed.pan_up = is_pressed;
                        true
                    }
                    KeyCode::ArrowDown => {
                        self.pressed.pan_down = is_pressed;
                        true
                    }
                    KeyCode::ArrowLeft => {
                        self.pressed.pan_left = is_pressed;
                        true
                    }
                    KeyCode::ArrowRight => {
                        self.pressed.pan_right = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub(crate) fn update_camera(&self, camera: &mut Camera) {
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when the camera gets too close to the
        // center of the scene.
        if self.pressed.zoom_in && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.pressed.zoom_out {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the forward/backward is pressed.
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        // Rescale the distance between the target and the eye so
        // that it doesn't change. The eye, therefore, still
        // lies on the circle made by the target and eye.

        if self.pressed.orbit_down {
            camera.eye = camera.target - (forward + camera.up * self.speed).normalize() * forward_mag;
        }
        if self.pressed.orbit_up {
            camera.eye = camera.target - (forward - camera.up * self.speed).normalize() * forward_mag;
        }
        if self.pressed.orbit_left {
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.pressed.orbit_right {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }

        // if self.pressed.pan_down {
        //     camera.target = camera.target - camera.up * self.speed;
        // }

        // if self.pressed.pan_up {
        //     camera.target = camera.target + camera.up * self.speed;
        // }
    }
}