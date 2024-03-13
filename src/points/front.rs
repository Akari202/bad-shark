use log::info;
use vec_utils::vec3d::Vec3d;
use crate::points::members::{AArm, InnerCV, Damper, TieRod, Wheel};

#[derive(Debug)]
pub struct FrontPoints {
    pub upper_wishbone: AArm,
    pub lower_wishbone: AArm,
    pub damper: Damper,
    pub steering: TieRod,
    pub wheel: Wheel,
    pub axle: InnerCV
}

impl FrontPoints {
    pub fn caster(&self) -> f64 {
        let castor_axis = Vec3d::new_from_to(
            &self.lower_wishbone.outer_ball_joint,
            &self.upper_wishbone.outer_ball_joint
        ).project_onto_plane(&Vec3d::j());
        castor_axis.angle_to(&Vec3d::k())
    }

    pub fn track_width(&self) -> f64 {
        self.wheel.center.y * 2.0
    }
}
