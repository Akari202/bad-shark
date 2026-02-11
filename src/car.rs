use std::error::Error;

use itertools::Itertools;
use log::info;
use vec_utils::angle::AngleDegrees;

use crate::car::front::Front;
use crate::car::rear::Rear;
use crate::graphics::vertex::Vertex;

mod front;
mod members;
mod rear;
pub(crate) mod test_car;
mod wheel;

#[derive(PartialEq, Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Car {
    pub front: Front,
    rear: Rear
}

impl Car {
    pub(crate) fn get_vertex_data(&self, color: [f32; 3]) -> Vec<(Vec<Vertex>, Vec<u16>)> {
        vec![
            self.front.get_vertex_data(color),
            self.rear.get_vertex_data(color),
        ]
    }

    pub fn rotate(&mut self, angle: AngleDegrees) -> Result<(), Box<dyn Error>> {
        self.rotate_front(angle)?;
        self.rotate_rear(angle)?;
        Ok(())
    }

    fn rotate_front(&mut self, angle: AngleDegrees) -> Result<(), Box<dyn Error>> {
        self.front.rotate_upper_aarm(angle)?;
        Ok(())
    }

    fn rotate_rear(&mut self, angle: AngleDegrees) -> Result<(), Box<dyn Error>> {
        self.rear.rotate_harm(angle)?;
        Ok(())
    }
}
