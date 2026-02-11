#![allow(unused_imports)]
use crate::car::test_car::get_test_car;
use crate::graphics::vertex::Vertex;

mod app;
pub mod car;
pub(crate) mod graphics;
pub use app::BSApp;

pub const ANGLE_EPSILON_DEGREES: f64 = 0.1;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);
