use vec_utils::angle::AngleRadians;
use vec_utils::matrix::matrix3x3;
use vec_utils::vec3d::Vec3d;

pub mod a_arm;
pub mod h_arm;
pub mod link;

pub trait Member {
    fn angles(&self) -> Vec3d;

    // NOTE: This only applies the internal rotation to internal link coordinates
    fn rotate_to_internal(&self, vector: &Vec3d) -> Vec3d {
        let angles = self.angles();
        // yaw
        let xy_angle = AngleRadians::new(angles.x);
        // pitch
        let yz_angle = AngleRadians::new(0.0);
        // roll
        let zx_angle = AngleRadians::new(angles.z);
        let rotation_matrix = [
            [xy_angle.cos() * yz_angle.cos(), xy_angle.sin() * yz_angle.sin() * zx_angle.cos() - xy_angle.cos() * zx_angle.sin(), xy_angle.cos() * yz_angle.sin() * zx_angle.cos() + xy_angle.sin() * zx_angle.sin()],
            [xy_angle.cos() * yz_angle.sin(), xy_angle.sin() * yz_angle.sin() * zx_angle.sin() + xy_angle.cos() * zx_angle.cos(), xy_angle.cos() * yz_angle.sin() * zx_angle.sin() - xy_angle.sin() * zx_angle.cos()],
            [-1.0 * yz_angle.sin(), xy_angle.sin() * yz_angle.cos(), xy_angle.cos() * yz_angle.cos()]
        ];
        matrix3x3::mul(&rotation_matrix, vector)
    }

    // NOTE: Only undoes the transformation to internal coordinates
    fn rotate_from_internal(&self, vector: &Vec3d) -> Vec3d {
        let angles = self.angles();
        // yaw
        let xy_angle = AngleRadians::new(-1.0 * angles.x);
        // pitch
        let yz_angle = AngleRadians::new(0.0);
        // roll
        let zx_angle = AngleRadians::new(-1.0 * angles.z);
        let rotation_matrix = [
            [xy_angle.cos() * yz_angle.cos(), xy_angle.sin() * yz_angle.sin() * zx_angle.cos() - xy_angle.cos() * zx_angle.sin(), xy_angle.cos() * yz_angle.sin() * zx_angle.cos() + xy_angle.sin() * zx_angle.sin()],
            [xy_angle.cos() * yz_angle.sin(), xy_angle.sin() * yz_angle.sin() * zx_angle.sin() + xy_angle.cos() * zx_angle.cos(), xy_angle.cos() * yz_angle.sin() * zx_angle.sin() - xy_angle.sin() * zx_angle.cos()],
            [-1.0 * yz_angle.sin(), xy_angle.sin() * yz_angle.cos(), xy_angle.cos() * yz_angle.cos()]
        ];
        matrix3x3::mul(&rotation_matrix, vector)
    }
}