use vec_utils::angle::AngleRadians;
use vec_utils::matrix::matrix3x3;
use vec_utils::vec3d::Vec3d;
use crate::car::members::a_arm::AArm;
use crate::car::members::link::Link;
use crate::car::members::Member;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HArm {
    // NOTE: the applied rotation is stored in the j spot
    pub angles: Vec3d,
    pub rear: f64,
    pub outer_front: Vec3d,
    pub outer_rear: Vec3d,
    pub damper: Vec3d
}

impl HArm {
    pub fn new(angles: Vec3d, rear: f64, outer_front: Vec3d, outer_rear: Vec3d, damper: Vec3d) -> Self {
        Self {
            angles,
            rear,
            outer_front,
            outer_rear,
            damper
        }
    }

    pub fn from_global(front: Vec3d, rear: Vec3d, outer_front: Vec3d, outer_rear: Vec3d, damper: Vec3d) -> Self {
        let rear = rear - front;
        // yaw
        let xy_angle = Vec3d::i().angle_to(&rear.project_onto_plane(&Vec3d::k()));
        // pitch
        let yz_angle = AngleRadians::new(0.0);
        // roll
        let zx_angle = Vec3d::i().angle_to(&rear.project_onto_plane(&Vec3d::j()));
        let rotation_matrix = [
            [xy_angle.cos() * yz_angle.cos(), xy_angle.sin() * yz_angle.sin() * zx_angle.cos() - xy_angle.cos() * zx_angle.sin(), xy_angle.cos() * yz_angle.sin() * zx_angle.cos() + xy_angle.sin() * zx_angle.sin()],
            [xy_angle.cos() * yz_angle.sin(), xy_angle.sin() * yz_angle.sin() * zx_angle.sin() + xy_angle.cos() * zx_angle.cos(), xy_angle.cos() * yz_angle.sin() * zx_angle.sin() - xy_angle.sin() * zx_angle.cos()],
            [-1.0 * yz_angle.sin(), xy_angle.sin() * yz_angle.cos(), xy_angle.cos() * yz_angle.cos()]
        ];

        let rear = matrix3x3::mul(&rotation_matrix, &rear);
        let outer_front = matrix3x3::mul(&rotation_matrix, &(outer_front - front));
        let outer_rear = matrix3x3::mul(&rotation_matrix, &(outer_rear - front));
        let damper = matrix3x3::mul(&rotation_matrix, &(damper - front));
        Self::new(Vec3d::new(xy_angle.into(), 0.0, zx_angle.into()), rear.magnitude(), outer_front, outer_rear, damper)
    }

    // returns (front, rear, outer_front, outer_rear, damper)
    pub fn get_global(&self, datum: &Vec3d) -> (Vec3d, Vec3d, Vec3d, Vec3d, Vec3d) {
        (
            datum.clone(),
            self.rotate_from_internal(&(self.rear * Vec3d::i())) + datum,
            self.rotate_from_internal(&self.outer_front) + datum,
            self.rotate_from_internal(&self.outer_rear) + datum,
            self.rotate_from_internal(&self.damper) + datum
        )
    }

    pub fn rotate(&self, epsilon: AngleRadians) -> Self {
        let rotation_matrix = [
            [1.0, 0.0, 0.0],
            [0.0, epsilon.cos(), -1.0 * epsilon.sin()],
            [0.0, epsilon.sin(), epsilon.cos()]
        ];
        let outer_front = matrix3x3::mul(&rotation_matrix, &self.outer_front);
        let outer_rear = matrix3x3::mul(&rotation_matrix, &self.outer_rear);
        let damper = matrix3x3::mul(&rotation_matrix, &self.damper);
        Self {
            angles: self.angles + f64::from(epsilon) * Vec3d::j(),
            rear: self.rear,
            outer_front,
            outer_rear,
            damper
        }
    }
}

impl Member for HArm {
    fn angles(&self) -> Vec3d {
        self.angles
    }
}
