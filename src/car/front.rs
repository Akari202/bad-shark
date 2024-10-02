use itertools::Itertools;
use vec_utils::angle::{AngleDegrees, AngleRadians};
use vec_utils::vec3d::Vec3d;
use vec_utils::geometry::sphere::Sphere;
use vec_utils::geometry::circle::Circle;
use vec_utils::geometry::intersection::sphere_circle;
use crate::ANGLE_EPSILON_DEGREES;
use crate::car::members::a_arm::AArm;

#[derive(Debug)]
pub struct Front {
    pub upper_datum: Vec3d,
    pub upper: AArm,
    pub lower_datum: Vec3d,
    pub lower: AArm,
    pub damper_body: Vec3d
}

impl Front {
    pub fn motion_ratios(&self) -> Vec<f64> {
        let angle_steps = (360.0 / ANGLE_EPSILON_DEGREES) as usize;
        let damper = self.damper_body - self.upper_datum;
        (0..angle_steps).map(|i| {
            let angle_epsilon = AngleDegrees::new(ANGLE_EPSILON_DEGREES * i as f64);
            self.upper.rotate(angle_epsilon.into())
        }).circular_tuple_windows::<(_, _)>().map(|(i, j)| {
            let z_delta = i.outer.z - j.outer.z;
            let d_delta = damper.distance_to(&i.damper.unwrap()) -
                damper.distance_to(&j.damper.unwrap());
            d_delta / z_delta
        }).collect()
    }

    pub fn caster_angle(&self) -> AngleRadians {
        let caster = self.outer_upright_mounting_vec_global()
            .project_onto_plane(&Vec3d::j());
        caster.angle_to(&Vec3d::k())
    }

    fn outer_upright_mounting_vec_global(&self) -> Vec3d {
        let upper = self.upper_datum + self.upper.unrotate_from_internal(&self.upper.outer);
        let lower = self.lower_datum + self.lower.unrotate_from_internal(&self.lower.outer);
        Vec3d::new_from_to(&lower, &upper)
    }

    pub fn outer_upright_mounting_distance(&self) -> f64 {
        self.outer_upright_mounting_vec_global().magnitude()
    }

    pub fn rotate_upper_aarm(&self) -> Option<Self> {
        let rotated_upper = self.upper.rotate(AngleDegrees::new(ANGLE_EPSILON_DEGREES).into());
        let upper_outer_g = rotated_upper.unrotate_from_internal(&rotated_upper.outer) + self.upper_datum;
        let upright_sphere_l = Sphere::new(
            &self.lower.rotate_to_internal(&(upper_outer_g - self.lower_datum)),
            self.outer_upright_mounting_distance()
        );
        let aarm_rotation_center_l = self.lower.outer.x * Vec3d::i();
        let aarm_circle_l = Circle::new(
            &aarm_rotation_center_l,
            aarm_rotation_center_l.distance_to(&self.lower.outer),
            &Vec3d::i()
        );
        let intersection_l = sphere_circle(&upright_sphere_l, &aarm_circle_l)?;
        // dbg!(self.lower);
        // dbg!(upright_sphere_l);
        // dbg!(aarm_circle_l);
        // dbg!(intersection_l);
        let angle = intersection_l.0
            .project_onto_plane(&Vec3d::i())
            .angle_to(&self.lower.outer.project_onto_plane(&Vec3d::i()));
        dbg!(angle);
        // println!("Angle lower aarm moves: {:.3}", angle);
        let rotated_lower = self.lower.rotate(angle);
        Some(Self {
            upper_datum: self.upper_datum,
            upper: rotated_upper,
            lower_datum: self.lower_datum,
            lower: rotated_lower,
            damper_body: self.damper_body
        })
    }

    pub fn print_coordinates(&self) {

    }
}
