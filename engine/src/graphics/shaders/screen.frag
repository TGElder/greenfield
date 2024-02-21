#version 330

in vec2 fragment_canvas_position;

uniform sampler2D canvas;

out vec4 color;

void main() {
    color = texture(canvas, fragment_canvas_position);
}