use vec_utils::vec3d::Vec3d;

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Link {
    pub angles: Vec3d,
    pub outer: Vec3d
}