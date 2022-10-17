mod canvas;
mod primitive;

use std::error::Error;

pub use canvas::*;
pub use primitive::*;

pub struct Programs {
    pub screen: glium::Program,
    pub primitive: glium::Program,
}

impl Programs {
    pub fn new(display: &dyn glium::backend::Facade) -> Result<Programs, Box<dyn Error>> {
        Ok(Programs {
            primitive: primitive::program(display)?,
            screen: canvas::program(display)?,
        })
    }
}
