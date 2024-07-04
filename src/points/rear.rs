use vec_utils::vec3d::Vec3d;
use crate::points::members::{InnerCV, Damper, HArm, RadiusRod, Wheel};
use crate::units::Distance;

#[derive(Debug)]
pub struct RearPoints {
    pub wishbone: HArm,
    pub radius_rod: RadiusRod,
    pub damper: Damper,
    pub wheel: Wheel,
    pub axle: InnerCV
}

impl RearPoints {}
