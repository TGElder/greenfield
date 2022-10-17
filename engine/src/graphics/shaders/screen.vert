#version 330

in vec2 screen_position;
in vec2 canvas_position;

out vec2 fragment_canvas_position;
                
void main() {
    fragment_canvas_position = canvas_position;
    gl_Position = vec4(screen_position, 0.0, 1.0);
}