use vec_utils::angle::AngleRadians;
use vec_utils::matrix::matrix3x3;
use vec_utils::vec3d::Vec3d;
use crate::car::members::Member;

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Link {
    pub angles: Vec3d,
    pub outer: Vec3d
}

impl Link {
    pub fn new(angles: Vec3d, outer: Vec3d) -> Self {
        Self {
            angles,
            outer
        }
    }

    pub fn from_global(inner: Vec3d, outer: Vec3d) -> Self {
        let outer = outer - inner;
        // yaw
        let xy_angle = Vec3d::i().angle_to(&outer.project_onto_plane(&Vec3d::k()));
        // pitch
        let yz_angle = AngleRadians::new(0.0);
        // roll
        let zx_angle = Vec3d::i().angle_to(&outer.project_onto_plane(&Vec3d::j()));

        let rotation_matrix = [
            [xy_angle.cos() * yz_angle.cos(), xy_angle.sin() * yz_angle.sin() * zx_angle.cos() - xy_angle.cos() * zx_angle.sin(), xy_angle.cos() * yz_angle.sin() * zx_angle.cos() + xy_angle.sin() * zx_angle.sin()],
            [xy_angle.cos() * yz_angle.sin(), xy_angle.sin() * yz_angle.sin() * zx_angle.sin() + xy_angle.cos() * zx_angle.cos(), xy_angle.cos() * yz_angle.sin() * zx_angle.sin() - xy_angle.sin() * zx_angle.cos()],
            [-1.0 * yz_angle.sin(), xy_angle.sin() * yz_angle.cos(), xy_angle.cos() * yz_angle.cos()]
        ];

        let outer = matrix3x3::mul(&rotation_matrix, &outer);
        Self::new(Vec3d::new(xy_angle.into(), 0.0, zx_angle.into()), outer)
    }

    pub fn get_global(&self, datum: &Vec3d) -> (Vec3d, Vec3d) {
        (
            *datum,
            self.rotate_from_internal(&self.outer) + datum
        )
    }

    pub fn rotate(&self, epsilon: AngleRadians) -> Self {
        let rotation_matrix = [
            [1.0, 0.0, 0.0],
            [0.0, epsilon.cos(), -1.0 * epsilon.sin()],
            [0.0, epsilon.sin(), epsilon.cos()]
        ];
        let outer = matrix3x3::mul(&rotation_matrix, &self.outer);
        Self {
            angles: self.angles + f64::from(epsilon) * Vec3d::j(),
            outer
        }
    }
}

impl Member for Link {
    fn angles(&self) -> Vec3d {
        self.angles
    }
}