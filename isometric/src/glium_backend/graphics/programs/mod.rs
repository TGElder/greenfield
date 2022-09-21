mod canvas;
mod primitive;

pub use canvas::*;
pub use primitive::*;

pub struct Programs {
    pub screen: glium::Program,
    pub primitive: glium::Program,
}

impl Programs {
    pub fn new(display: &dyn glium::backend::Facade) -> Programs {
        Programs {
            primitive: primitive::program(display),
            screen: canvas::program(display),
        }
    }
}
