use vec_utils::angle::AngleDegrees;
use vec_utils::geometry::circle::Circle;
use vec_utils::geometry::intersection::sphere_circle;
use vec_utils::geometry::plane::Plane;
use vec_utils::geometry::sphere::Sphere;
use vec_utils::vec3d::Vec3d;

use crate::points::members::{AArm, Damper, InnerCV, TieRod, Wheel};

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
    pub fn caster(&self) -> AngleDegrees {
        let castor_axis = Vec3d::new_from_to(
            &self.lower_wishbone.outer_ball_joint,
            &self.upper_wishbone.outer_ball_joint
        ).project_onto_plane(&Vec3d::j());
        castor_axis.angle_to(&Vec3d::k()).into()
    }

    // I would like to apologize for this appalling mess of variable names
    pub fn upper_wishbone_angle_from_damper_extension(&self, extension: f64) -> Result<AngleDegrees, FrontPointsError> {
        assert!(1.0 >= extension && extension >= 0.0, "extension must be between 0 and 1");

        let damper_extension = self.damper.eye_to_eye - self.damper.stroke * (1.0 - extension);
        let damper_sphere = Sphere::new(&self.damper.body, damper_extension);
        let upper_wishbone_damper_mount_radius = self.damper.wishbone.distance_to_line(&self.upper_wishbone.front_pivot, &self.upper_wishbone.rear_pivot);
        let upper_wishbone_damper_mount_center_of_rotation = self.damper.wishbone.project_onto_line(
            &self.upper_wishbone.front_pivot,
            &self.upper_wishbone.rear_pivot
        );

        let upper_wishbone_damper_mount_rotation_plane = Plane::from_point(
            &Vec3d::new_from_to(
                &self.upper_wishbone.front_pivot,
                &self.upper_wishbone.rear_pivot
            ),
            &self.damper.wishbone
        );

        let upper_wishbone_damper_mount_rotation_axis = self.upper_wishbone.rotation_axis();
        let upper_wishbone_damper_mount_circle = Circle::new(
            &upper_wishbone_damper_mount_center_of_rotation,
            upper_wishbone_damper_mount_radius,
            &upper_wishbone_damper_mount_rotation_axis
        );
        let intersection = sphere_circle(&damper_sphere, &upper_wishbone_damper_mount_circle);
        dbg!(intersection);
        Ok(AngleDegrees::new(180.0))
    }
}

#[derive(Debug)]
pub enum FrontPointsError {
    DamperWishboneOutOfAlignment,
    ExtensionOutOfRange
}
