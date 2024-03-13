use vec_utils::vec3d::Vec3d;
use crate::points::front::FrontPoints;
use crate::points::members::{AArm, InnerCV, Damper, HArm, Link, Wheel};
use crate::points::rear::RearPoints;

pub mod front;
pub mod rear;
pub mod members;

#[derive(Debug)]
pub struct CarPoints {
    pub front: FrontPoints,
    pub rear: RearPoints
}

impl CarPoints {
    pub fn wheelbase(&self) -> f64 {
        self.rear.wheel.center.x - self.front.wheel.center.x
    }

    pub fn track_width(&self) -> (f64, f64) {
        (self.front.track_width(), self.rear.track_width())
    }

    pub fn camber(&self) -> (f64, f64) {
        (self.front.wheel.camber(), self.rear.wheel.camber())
    }
}

