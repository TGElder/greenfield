use std::error::Error;

pub struct Programs {
    pub primitive: glium::Program,
    pub screen: glium::Program,
    pub billboard: glium::Program,
    pub overlay_primitive: glium::Program,
    pub instanced_primitive: glium::Program,
}

impl Programs {
    pub fn new(facade: &dyn glium::backend::Facade) -> Result<Programs, Box<dyn Error>> {
        Ok(Programs {
            primitive: from_source(
                facade,
                include_str!("../../graphics/shaders/primitive.vert"),
                include_str!("../../graphics/shaders/primitive.frag"),
                None,
            )?,
            overlay_primitive: from_source(
                facade,
                include_str!("../../graphics/shaders/overlay_primitive.vert"),
                include_str!("../../graphics/shaders/overlay_primitive.frag"),
                None,
            )?,
            instanced_primitive: from_source(
                facade,
                include_str!("../../graphics/shaders/instanced_primitive.vert"),
                include_str!("../../graphics/shaders/instanced_primitive.frag"),
                None,
            )?,
            screen: from_source(
                facade,
                include_str!("../../graphics/shaders/screen.vert"),
                include_str!("../../graphics/shaders/screen.frag"),
                None,
            )?,
            billboard: from_source(
                facade,
                include_str!("../../graphics/shaders/billboard.vert"),
                include_str!("../../graphics/shaders/billboard.frag"),
                None,
            )?,
        })
    }
}

// This replicates glium::Program::from_source with outputs_srgb: false
pub fn from_source<'a, F>(
    facade: &F,
    vertex_shader: &'a str,
    fragment_shader: &'a str,
    geometry_shader: Option<&'a str>,
) -> Result<glium::Program, glium::ProgramCreationError>
where
    F: glium::backend::Facade + ?Sized,
{
    glium::Program::new(
        facade,
        glium::program::ProgramCreationInput::SourceCode {
            vertex_shader,
            fragment_shader,
            geometry_shader,
            tessellation_control_shader: None,
            tessellation_evaluation_shader: None,
            transform_feedback_varyings: None,
            outputs_srgb: false,
            uses_point_size: false,
        },
    )
}
