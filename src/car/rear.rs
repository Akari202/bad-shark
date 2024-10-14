use std::error::Error;
use cgmath::num_traits::real::Real;
use itertools::Itertools;
use vec_utils::angle::{AngleDegrees, AngleRadians};
use vec_utils::vec3d::Vec3d;
use vec_utils::geometry::sphere::Sphere;
use vec_utils::geometry::circle::Circle;
use vec_utils::geometry::intersection::sphere_circle;
use crate::ANGLE_EPSILON_DEGREES;
use crate::car::front::Front;
use crate::car::members::h_arm::HArm;
use crate::car::members::link::Link;
use crate::graphics::vertex::Vertex;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rear {
    pub harm_datum: Vec3d,
    pub harm: HArm,
    pub camber_link_datum: Vec3d,
    pub camber_link: Link,
    pub damper_body: Vec3d,
    pub upright_distance: f64
}

impl Rear {
    pub fn motion_ratios(&self) -> Vec<f64> {
        let angle_steps = (360.0 / ANGLE_EPSILON_DEGREES) as usize;
        let damper = self.damper_body - self.harm_datum;
        (0..angle_steps).map(|i| {
            let angle_epsilon = AngleDegrees::new(ANGLE_EPSILON_DEGREES * i as f64);
            self.harm.rotate(angle_epsilon.into())
        }).circular_tuple_windows::<(_, _)>().map(|(i, j)| {
            let z_delta = i.outer_rear.z - j.outer_rear.z;
            let d_delta = damper.distance_to(&i.damper) -
                damper.distance_to(&j.damper);
            d_delta / z_delta
        }).collect()
    }

    // fn outer_upright_mounting_vec_global(&self) -> Vec3d {
    //     let upper = self.upper_datum + self.upper.unrotate_from_internal(&self.upper.outer);
    //     let lower = self.lower_datum + self.lower.unrotate_from_internal(&self.lower.outer);
    //     Vec3d::new_from_to(&lower, &upper)
    // }
    //
    // pub fn outer_upright_mounting_distance(&self) -> f64 {
    //     self.outer_upright_mounting_vec_global().magnitude()
    // }

    pub fn rotate_harm(&mut self, angle: AngleDegrees) -> Result<(), Box<dyn Error>> {
        self.harm = self.harm.rotate(angle.into());
        Ok(())
    }

    // pub fn rotate_upper_aarm(&mut self, angle: AngleDegrees) -> Result<(), Box<dyn Error>> {
    //     let rotated_upper = self.upper.rotate(angle.into());
    //     let upper_outer_g = rotated_upper.unrotate_from_internal(&rotated_upper.outer) + self.upper_datum;
    //     let upright_sphere_l = Sphere::new(
    //         &self.lower.rotate_to_internal(&(upper_outer_g - self.lower_datum)),
    //         self.upright_distance
    //     );
    //     let aarm_rotation_center_l = self.lower.outer.x * Vec3d::i();
    //     let aarm_circle_l = Circle::new(
    //         &aarm_rotation_center_l,
    //         aarm_rotation_center_l.distance_to(&self.lower.outer),
    //         &Vec3d::i()
    //     );
    //     let intersection_l = sphere_circle(&upright_sphere_l, &aarm_circle_l)
    //         .ok_or("Intesection Error")?;
    //     let lower_angle_1 = intersection_l.1
    //         .project_onto_plane(&Vec3d::i())
    //         .angle_to(
    //             &self
    //                 .lower
    //                 .outer
    //                 .project_onto_plane(&Vec3d::i())
    //         );
    //     let lower_angle_2 = intersection_l.0
    //         .project_onto_plane(&Vec3d::i())
    //         .angle_to(
    //             &self
    //                 .lower
    //                 .outer
    //                 .project_onto_plane(&Vec3d::i())
    //         );
    //     let lower_angle = lower_angle_1.min(lower_angle_2) * f64::from(angle.to_radians()).signum();
    //     println!("Upper AArm angle change: {}, Lower AArm angle change: {}", angle, lower_angle.to_degrees());
    //     self.upper = rotated_upper;
    //     self.lower = self.lower.rotate(lower_angle);
    //     Ok(())
    // }

    pub(crate) fn get_vertex_data(&self, color: [f32; 3]) -> (Vec<Vertex>, Vec<u16>) {
        let harm = self.harm.get_global(&self.harm_datum);
        let camber_link = self.camber_link.get_global(&self.camber_link_datum);
        let vertex_data = vec![
            self.damper_body, harm.0, harm.1, harm.2, harm.3, harm.4, camber_link.0, camber_link.1
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
                0, 10, 2, 6, 6, 8, 8, 4, 12, 14,
                1, 11, 3, 7, 7, 9, 9, 5, 13, 15
            ]
        )
    }
}
