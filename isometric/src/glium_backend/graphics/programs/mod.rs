mod primitive;
mod screen;

pub use primitive::*;
pub use screen::*;

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
            screen: screen::program(display),
        }
    }
}
