use std::error::Error;

const VERTEX_SHADER: &str = r#"
    #version 330

    in uint id;
    in vec3 position;
    in vec3 color;

    out vec3 fragment_color;
    flat out float id_in_float;

    uniform mat4 transform;

    void main() {
        fragment_color = color;
        id_in_float = uintBitsToFloat(id);
        gl_Position = transform * vec4(position.x, position.y, position.z, 1.0);
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    #version 330

    in vec3 fragment_color;
    flat in float id_in_float;

    out vec4 color;

    void main() {
        color = vec4(fragment_color, 1.0);
    }
"#;

pub fn program(display: &dyn glium::backend::Facade) -> Result<glium::Program, Box<dyn Error>> {
    Ok(glium::Program::from_source(
        display,
        VERTEX_SHADER,
        FRAGMENT_SHADER,
        None,
    )?)
}
