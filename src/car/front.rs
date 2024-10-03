use cgmath::num_traits::real::Real;
use itertools::Itertools;
use vec_utils::angle::{AngleDegrees, AngleRadians};
use vec_utils::vec3d::Vec3d;
use vec_utils::geometry::sphere::Sphere;
use vec_utils::geometry::circle::Circle;
use vec_utils::geometry::intersection::sphere_circle;
use crate::{Vertex, ANGLE_EPSILON_DEGREES};
use crate::car::members::a_arm::AArm;

#[derive(Debug, Copy, Clone)]
pub struct Front {
    pub upper_datum: Vec3d,
    pub upper: AArm,
    pub lower_datum: Vec3d,
    pub lower: AArm,
    pub damper_body: Vec3d,
    pub upright_distance: f64
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

    pub fn rotate_upper_aarm(&self, angle: AngleDegrees) -> Option<Self> {
        let rotated_upper = self.upper.rotate(angle.into());
        // let rotated_lower = self.lower.rotate(angle.into());
        let upper_outer_g = rotated_upper.unrotate_from_internal(&rotated_upper.outer) + self.upper_datum;
        let upright_sphere_l = Sphere::new(
            &self.lower.rotate_to_internal(&(upper_outer_g - self.lower_datum)),
            self.upright_distance
        );
        let aarm_rotation_center_l = self.lower.outer.x * Vec3d::i();
        let aarm_circle_l = Circle::new(
            &aarm_rotation_center_l,
            aarm_rotation_center_l.distance_to(&self.lower.outer),
            &Vec3d::i()
        );
        let intersection_l = sphere_circle(&upright_sphere_l, &aarm_circle_l)?;
        let lower_angle_1 = intersection_l.1
            .project_onto_plane(&Vec3d::i())
            .angle_to(
                &self
                    .lower
                    .outer
                    .project_onto_plane(&Vec3d::i())
            );
        let lower_angle_2 = intersection_l.0
            .project_onto_plane(&Vec3d::i())
            .angle_to(
                &self
                    .lower
                    .outer
                    .project_onto_plane(&Vec3d::i())
            );
        let lower_angle = lower_angle_1.min(lower_angle_2) * f64::from(angle.to_radians()).signum();
        println!("Upper AArm angle change: {}, Lower AArm angle change: {}", angle, lower_angle.to_degrees());
        let rotated_lower = self.lower.rotate(lower_angle);
        Some(Self {
            upper_datum: self.upper_datum,
            upper: rotated_upper,
            lower_datum: self.lower_datum,
            lower: rotated_lower,
            damper_body: self.damper_body,
            upright_distance: self.upright_distance
        })
    }

    pub fn print_coordinates(&self) {

    }

    pub(crate) fn get_vertex_data(&self, color: [f32; 3]) -> (Vec<Vertex>, Vec<u16>) {
        let upper = self.upper.get_global(&self.upper_datum);
        let lower = self.lower.get_global(&self.lower_datum);
        let vertex_data = vec![
            upper.0, upper.1, upper.2, lower.0, lower.1, lower.2, upper.3.unwrap(), self.damper_body
        ];

        (
            vertex_data.iter().map(|i| {
                let scaled = Vertex::from_vec3d(i, color).scale(500.0);
                vec![
                    scaled.mirror(),
                    scaled
                ]
            }).concat(),
            vec![
                0, 4, 2, 4, 6, 10, 8, 10, 12, 14,
                1, 5, 3, 5, 7, 11, 9, 11, 13, 15
                // 0, 2, 1, 2, 3, 5, 4, 5, 6, 7
            ]
        )
    }
}
