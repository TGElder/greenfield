const VERTEX_SHADER: &str = r#"
    #version 330

    in vec2 screen_position;
    in vec2 canvas_position;

    out vec2 fragment_canvas_position;
                    
    void main() {
        fragment_canvas_position = canvas_position;
        gl_Position = vec4(screen_position, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    #version 330

    in vec2 fragment_canvas_position;
    
    out vec4 color;
    
    uniform sampler2D canvas;
    
    void main() {
        color = texture(canvas, fragment_canvas_position);
    }
"#;

pub fn program(display: &dyn glium::backend::Facade) -> glium::Program {
    glium::Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap()
}
