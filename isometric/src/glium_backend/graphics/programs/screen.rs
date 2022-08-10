const VERTEX_SHADER: &str = r#"
        #version 330

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;
                        
        void main() {
            v_tex_coords = tex_coords;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

const FRAGMENT_SHADER: &str = r#"
        #version 330

        in vec2 v_tex_coords;
        out vec4 color;
        
        uniform sampler2D tex;
        
        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

pub fn program<F>(display: &F) -> glium::Program
where
    F: glium::backend::Facade,
{
    glium::Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap()
}
