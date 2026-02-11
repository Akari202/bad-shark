use cgmath::{InnerSpace, Quaternion, Rotation, Rotation3};
use eframe::egui::Vec2;
use eframe::wgpu;
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
    pub(crate) zfar: f32
}

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        OPENGL_TO_WGPU_MATRIX * proj * view
    }

    pub fn orbit(&mut self, delta: Vec2) {
        let rel_eye = self.eye - self.target;

        let yaw_speed = -0.005;
        let pitch_speed = -0.005;

        let yaw_rot = Quaternion::from_axis_angle(self.up, cgmath::Rad(delta.x * yaw_speed));

        let right = self.up.cross(rel_eye).normalize();
        let pitch_rot = Quaternion::from_axis_angle(right, cgmath::Rad(delta.y * pitch_speed));

        let new_rel_eye = yaw_rot * pitch_rot * rel_eye;
        self.eye = self.target + new_rel_eye;
    }

    pub fn zoom(&mut self, delta: f32) {
        let forward = self.eye - self.target;
        let distance = forward.magnitude();

        let zoom_factor = 0.005;
        let new_distance = distance - (delta * zoom_factor);

        let clamped_distance = new_distance.clamp(self.znear * 2.0, self.zfar * 0.9);

        self.eye = self.target + (forward.normalize() * clamped_distance);
    }

    pub fn reset(&mut self) {
        self.eye = (5.0, 5.0, 5.0).into();
        self.target = (0.0, 0.0, 0.0).into();
    }

    pub fn pan(&mut self, delta: Vec2) {
        let pan_speed = 0.0025;
        let forward = (self.target - self.eye).normalize();

        let right = forward.cross(self.up).normalize();
        let actual_up = right.cross(forward).normalize();

        let offset = (right * -delta.x * pan_speed) + (actual_up * delta.y * pan_speed);

        self.eye += offset;
        self.target += offset;
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct CameraUniform {
    // We can't use cgmath with bytemuck directly, so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4]
}

impl CameraUniform {
    pub(crate) fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into()
        }
    }

    pub(crate) fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}
