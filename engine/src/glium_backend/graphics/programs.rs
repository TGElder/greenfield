use std::error::Error;

pub struct Programs {
    pub primitive: glium::Program,
    pub screen: glium::Program,
    pub billboard: glium::Program,
    pub overlay_primitive: glium::Program,
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
            overlay_primitive: glium::Program::from_source(
                facade,
                include_str!("../../graphics/shaders/overlay_primitive.vert"),
                include_str!("../../graphics/shaders/overlay_primitive.frag"),
                None,
            )?,
        })
    }
}
