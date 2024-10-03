use std::error::Error;
use itertools::Itertools;
use vec_utils::angle::AngleDegrees;
use crate::car::front::Front;
use crate::Vertex;

mod members;
mod front;
mod wheel;
pub(crate) mod test_car;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Car {
    front: Front
}

impl Car {
    pub(crate) fn get_vertex_data(&self, color: [f32; 3]) -> Vec<(Vec<Vertex>, Vec<u16>)> {
        vec![self.front.get_vertex_data(color)]
    }

    pub fn rotate_front(&mut self, angle: AngleDegrees) -> Result<(), Box<dyn Error>> {
        self.front.rotate_upper_aarm(angle)?;
        Ok(())
    }
}