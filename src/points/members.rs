use vec_utils::vec3d::Vec3d;

#[derive(Debug)]
pub struct AArm {
    pub front_pivot: Vec3d,
    pub rear_pivot: Vec3d,
    pub outer_ball_joint: Vec3d
}

#[derive(Debug)]
pub struct HArm {
    pub inner_front_pivot: Vec3d,
    pub inner_rear_pivot: Vec3d,
    pub outer_front_pivot: Vec3d,
    pub outer_rear_pivot: Vec3d
}

#[derive(Debug)]
pub struct Damper {
    pub body: Vec3d,
    pub wishbone: Vec3d,
    pub max_length: f64,
    pub min_length: f64
}

#[derive(Debug)]
pub struct Link {
    pub inner: Vec3d,
    pub outer: Vec3d
}

pub type TieRod = Link;
pub type RadiusRod = Link;

#[derive(Debug)]
pub struct Wheel {
    pub center: Vec3d,
    pub spindle: Vec3d
}

#[derive(Debug)]
pub struct InnerCV {
    pub center: Vec3d,
    pub axis: Vec3d
}

impl Link {
    pub fn length(&self) -> f64 {
        self.inner.distance_to(&self.outer)
    }
}

impl Wheel {
    pub fn camber(&self) -> f64 {
        let camber_axis = Vec3d::new_from_to(
            &self.spindle,
            &self.center
        ).project_onto_plane(&Vec3d::i());
        camber_axis.angle_to(&Vec3d::j())
    }
}
