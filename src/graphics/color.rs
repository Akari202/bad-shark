use vec_utils::vec3d::Vec3d;
use crate::graphics::vertex::Vertex;

pub type Color = [f32; 3];

pub const BLACK: Color = [0.0, 0.0, 0.0];
pub const WHITE: Color = [1.0, 1.0, 1.0];
pub const RED: Color = [1.0, 0.0, 0.0];
pub const GREEN: Color = [0.0, 1.0, 0.0];
pub const BLUE: Color = [0.0, 0.0, 1.0];
pub const DARK_GRAY: Color = [0.3, 0.3, 0.3];
pub const LIGHT_GRAY: Color = [0.8, 0.8, 0.8];
pub const MIDDLE: Color = [0.5, 0.5, 0.5];

pub(crate) fn coordinate_axis() -> (Vec<Vertex>, Vec<u16>) {
    (
        vec![
            Vertex::from_vec3d(&Vec3d::zero(), MIDDLE),
            Vertex::from_vec3d(&Vec3d::i(), [1.0, 0.2, 0.2]),
            Vertex::from_vec3d(&Vec3d::j(), [0.2, 1.0, 0.2]),
            Vertex::from_vec3d(&Vec3d::k(), [0.2, 0.2, 1.0])
        ],
        vec![
            0, 1, 0, 2, 0, 3
        ]
    )
}
