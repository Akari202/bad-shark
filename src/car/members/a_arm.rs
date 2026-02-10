use vec_utils::angle::AngleRadians;
use vec_utils::matrix::real::Matrix3x3;
use vec_utils::vec3d::Vec3d;

use crate::car::members::Member;

#[derive(Copy, Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AArm {
    // NOTE: the applied rotation is stored in the j spot
    pub angles: Vec3d,
    pub rear: f64,
    pub outer: Vec3d,
    pub damper: Option<Vec3d>
}

impl AArm {
    pub fn new(angles: Vec3d, rear: f64, outer: Vec3d, damper: Option<Vec3d>) -> Self {
        Self {
            angles,
            rear,
            outer,
            damper
        }
    }

    pub fn from_global(front: Vec3d, rear: Vec3d, outer: Vec3d, damper: Option<Vec3d>) -> Self {
        let rear = rear - front;
        // yaw
        let xy_angle = Vec3d::i().angle_to(&rear.project_onto_plane(&Vec3d::k()));
        // pitch
        let yz_angle = AngleRadians::new(0.0);
        // roll
        let zx_angle = Vec3d::i().angle_to(&rear.project_onto_plane(&Vec3d::j()));
        let rotation_matrix = Matrix3x3::from_nested_arr([
            [
                xy_angle.cos() * yz_angle.cos(),
                xy_angle.sin() * yz_angle.sin() * zx_angle.cos() - xy_angle.cos() * zx_angle.sin(),
                xy_angle.cos() * yz_angle.sin() * zx_angle.cos() + xy_angle.sin() * zx_angle.sin()
            ],
            [
                xy_angle.cos() * yz_angle.sin(),
                xy_angle.sin() * yz_angle.sin() * zx_angle.sin() + xy_angle.cos() * zx_angle.cos(),
                xy_angle.cos() * yz_angle.sin() * zx_angle.sin() - xy_angle.sin() * zx_angle.cos()
            ],
            [
                -1.0 * yz_angle.sin(),
                xy_angle.sin() * yz_angle.cos(),
                xy_angle.cos() * yz_angle.cos()
            ]
        ]);

        let rear = (rotation_matrix * rear.to_vmatrix()).to_vec3d();
        let outer = (rotation_matrix * (outer - front).to_vmatrix()).to_vec3d();
        let damper = if let Some(damper) = damper {
            Some((rotation_matrix * (damper - front).to_vmatrix()).to_vec3d())
        } else {
            None
        };
        Self::new(
            Vec3d::new(xy_angle.into(), 0.0, zx_angle.into()),
            rear.magnitude(),
            outer,
            damper
        )
    }

    // returns (front, rear, outer, Option<damper>)
    pub fn get_global(&self, datum: &Vec3d) -> (Vec3d, Vec3d, Vec3d, Option<Vec3d>) {
        (
            datum.clone(),
            self.rotate_from_internal(&(self.rear * Vec3d::i())) + datum,
            self.rotate_from_internal(&self.outer) + datum,
            if self.damper.is_none() {
                None
            } else {
                Some(self.rotate_from_internal(&self.damper.unwrap()) + datum)
            }
        )
    }

    pub fn rotate(&self, epsilon: AngleRadians) -> Self {
        let rotation_matrix = Matrix3x3::from_nested_arr([
            [1.0, 0.0, 0.0],
            [0.0, epsilon.cos(), -1.0 * epsilon.sin()],
            [0.0, epsilon.sin(), epsilon.cos()]
        ]);
        let outer = (rotation_matrix * self.outer.to_vmatrix()).to_vec3d();
        let damper = if let Some(damper) = self.damper {
            Some((rotation_matrix * damper.to_vmatrix()).to_vec3d())
        } else {
            None
        };
        Self {
            angles: self.angles + f64::from(epsilon) * Vec3d::j(),
            rear: self.rear,
            outer,
            damper
        }
    }
}

impl Member for AArm {
    fn angles(&self) -> Vec3d {
        self.angles
    }
}
