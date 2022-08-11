mod canvas;
mod primitive;

pub use canvas::*;
pub use primitive::*;

pub struct Programs {
    pub screen: glium::Program,
    pub primitive: glium::Program,
}

impl Programs {
    pub fn new<F>(display: &F) -> Programs
    where
        F: glium::backend::Facade,
    {
        Programs {
            primitive: primitive::program(display),
            screen: canvas::program(display),
        }
    }
}
