#version 330

in vec2 fragment_canvas_position;

out vec4 color;

uniform sampler2D canvas;

void main() {
    color = texture(canvas, fragment_canvas_position);
}