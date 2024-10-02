use vec_utils::angle::AngleRadians;
use vec_utils::matrix::matrix3x3;
use vec_utils::vec3d::Vec3d;

#[derive(Copy, Clone, Debug)]
pub struct AArm {
    // pub front: Vec3d,
    // pub rear: Vec3d,
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
        let rotation_matrix = [
            [xy_angle.cos() * yz_angle.cos(), xy_angle.sin() * yz_angle.sin() * zx_angle.cos() - xy_angle.cos() * zx_angle.sin(), xy_angle.cos() * yz_angle.sin() * zx_angle.cos() + xy_angle.sin() * zx_angle.sin()],
            [xy_angle.cos() * yz_angle.sin(), xy_angle.sin() * yz_angle.sin() * zx_angle.sin() + xy_angle.cos() * zx_angle.cos(), xy_angle.cos() * yz_angle.sin() * zx_angle.sin() - xy_angle.sin() * zx_angle.cos()],
            [-1.0 * yz_angle.sin(), xy_angle.sin() * yz_angle.cos(), xy_angle.cos() * yz_angle.cos()]
        ];

        let rear = matrix3x3::mul(&rotation_matrix, &rear);
        let outer = matrix3x3::mul(&rotation_matrix, &(outer - front));
        let damper = if let Some(damper) = damper {
            Some(matrix3x3::mul(&rotation_matrix, &(damper - front)))
        } else {
            None
        };
        Self::new(Vec3d::new(xy_angle.into(), 0.0, zx_angle.into()), rear.magnitude(), outer, damper)
    }

    // NOTE: This only applies the internal rotation to internal aarm coordinate
    pub fn rotate_to_internal(&self, vector: &Vec3d) -> Vec3d {
        // yaw
        let xy_angle = AngleRadians::new(self.angles.x);
        // pitch
        let yz_angle = AngleRadians::new(0.0);
        // roll
        let zx_angle = AngleRadians::new(self.angles.z);
        let rotation_matrix = [
            [xy_angle.cos() * yz_angle.cos(), xy_angle.sin() * yz_angle.sin() * zx_angle.cos() - xy_angle.cos() * zx_angle.sin(), xy_angle.cos() * yz_angle.sin() * zx_angle.cos() + xy_angle.sin() * zx_angle.sin()],
            [xy_angle.cos() * yz_angle.sin(), xy_angle.sin() * yz_angle.sin() * zx_angle.sin() + xy_angle.cos() * zx_angle.cos(), xy_angle.cos() * yz_angle.sin() * zx_angle.sin() - xy_angle.sin() * zx_angle.cos()],
            [-1.0 * yz_angle.sin(), xy_angle.sin() * yz_angle.cos(), xy_angle.cos() * yz_angle.cos()]
        ];
        matrix3x3::mul(&rotation_matrix, vector)
    }

    // NOTE: Only undoes the transformation to internal coordinates
    pub fn unrotate_from_internal(&self, vector: &Vec3d) -> Vec3d {
        // yaw
        let xy_angle = AngleRadians::new(-1.0 * self.angles.x);
        // pitch
        let yz_angle = AngleRadians::new(0.0);
        // roll
        let zx_angle = AngleRadians::new(-1.0 * self.angles.z);
        let rotation_matrix = [
            [xy_angle.cos() * yz_angle.cos(), xy_angle.sin() * yz_angle.sin() * zx_angle.cos() - xy_angle.cos() * zx_angle.sin(), xy_angle.cos() * yz_angle.sin() * zx_angle.cos() + xy_angle.sin() * zx_angle.sin()],
            [xy_angle.cos() * yz_angle.sin(), xy_angle.sin() * yz_angle.sin() * zx_angle.sin() + xy_angle.cos() * zx_angle.cos(), xy_angle.cos() * yz_angle.sin() * zx_angle.sin() - xy_angle.sin() * zx_angle.cos()],
            [-1.0 * yz_angle.sin(), xy_angle.sin() * yz_angle.cos(), xy_angle.cos() * yz_angle.cos()]
        ];
        matrix3x3::mul(&rotation_matrix, vector)
    }

    // returns (front, rear, outer, Option<damper>)
    pub fn get_global(&self, datum: &Vec3d) -> (Vec3d, Vec3d, Vec3d, Option<Vec3d>) {
        (
            datum.clone(),
            self.unrotate_from_internal(&(self.rear * Vec3d::i())) + datum,
            self.unrotate_from_internal(&self.outer) + datum,
            if self.damper.is_none() {
                None
            } else {
                Some(self.unrotate_from_internal(&self.damper.unwrap()) + datum)
            }
        )
    }

    pub fn rotate(&self, epsilon: AngleRadians) -> Self {
        let rotation_matrix = [
            [1.0, 0.0, 0.0],
            [0.0, epsilon.cos(), -1.0 * epsilon.sin()],
            [0.0, epsilon.sin(), epsilon.cos()]
        ];
        let outer = matrix3x3::mul(&rotation_matrix, &self.outer);
        let damper = if let Some(damper) = self.damper {
            Some(matrix3x3::mul(&rotation_matrix, &(damper)))
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