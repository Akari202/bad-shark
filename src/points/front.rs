use log::info;
use vec_utils::vec3d::Vec3d;
use crate::points::members::{AArm, InnerCV, Damper, TieRod, Wheel};
use crate::units::Angle;

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
    pub fn caster(&self) -> Angle {
        let castor_axis = Vec3d::new_from_to(
            &self.lower_wishbone.outer_ball_joint,
            &self.upper_wishbone.outer_ball_joint
        ).project_onto_plane(&Vec3d::j());
        Angle::from_radians(castor_axis.angle_to(&Vec3d::k()))
    }
}
