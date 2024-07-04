use vec_utils::vec3d::Vec3d;
use crate::points::front::FrontPoints;
use crate::points::members::{AArm, InnerCV, Damper, HArm, Link, Wheel};
use crate::points::rear::RearPoints;
use crate::units::{Angle, Distance};

pub mod front;
pub mod rear;
pub mod members;

#[derive(Debug)]
pub struct CarPoints {
    pub front: FrontPoints,
    pub rear: RearPoints
}

impl CarPoints {
    pub fn wheelbase(&self) -> Distance {
        Distance::from_millimeters(self.rear.wheel.center.x - self.front.wheel.center.x)
    }

    pub fn track_width(&self) -> (Distance, Distance) {
        (
            self.front.wheel.track_width(),
            self.rear.wheel.track_width()
        )
    }

    pub fn camber(&self) -> (Angle, Angle) {
        (self.front.wheel.camber(), self.rear.wheel.camber())
    }
}

