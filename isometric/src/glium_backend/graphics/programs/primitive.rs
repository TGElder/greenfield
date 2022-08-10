use glium::backend::glutin;

const VERTEX_SHADER: &str = r#"
#version 330

in vec3 position;
in uint id;
out float height;
flat out float id_in_float;
flat out int selected;

uniform uint selection;
uniform mat4 matrix;

void main() {
    height = position.z / 32.0;
    id_in_float = uintBitsToFloat(id);
    if (id == selection) {
        selected = 1;
    } else {
        selected = 0;
    }
    gl_Position = matrix * vec4(position.x, position.y, position.z, 1.0);
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 330

in float height;
flat in float id_in_float;
flat in int selected;
out vec4 color;

void main() {
    if (selected == 1) {
        color = vec4(1.0, 0.0, 0.0, id_in_float);
    } else {
        color = vec4(height, height, height, id_in_float);
    }
}
"#;

pub fn program<F>(display: &F) -> glium::Program
where
    F: glium::backend::Facade,
{
    glium::Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap()
}
