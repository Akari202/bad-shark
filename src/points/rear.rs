use vec_utils::vec3d::Vec3d;
use crate::points::members::{InnerCV, Damper, HArm, RadiusRod, Wheel};

#[derive(Debug)]
pub struct RearPoints {
    pub wishbone: HArm,
    pub radius_rod: RadiusRod,
    pub damper: Damper,
    pub wheel: Wheel,
    pub axle: InnerCV
}

impl RearPoints {
    pub fn track_width(&self) -> f64 {
        self.wheel.center.y * 2.0
    }
}
