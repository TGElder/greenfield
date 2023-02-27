use std::error::Error;

pub struct Programs {
    pub screen: glium::Program,
    pub billboard: glium::Program,
    pub primitive: glium::Program,
}

impl Programs {
    pub fn new(facade: &dyn glium::backend::Facade) -> Result<Programs, Box<dyn Error>> {
        Ok(Programs {
            primitive: glium::Program::from_source(
                facade,
                include_str!("../../graphics/shaders/primitive.vert"),
                include_str!("../../graphics/shaders/primitive.frag"),
                None,
            )?,
            billboard: glium::Program::from_source(
                facade,
                include_str!("../../graphics/shaders/billboard.vert"),
                include_str!("../../graphics/shaders/billboard.frag"),
                None,
            )?,
            screen: glium::Program::from_source(
                facade,
                include_str!("../../graphics/shaders/screen.vert"),
                include_str!("../../graphics/shaders/screen.frag"),
                None,
            )?,
        })
    }
}
