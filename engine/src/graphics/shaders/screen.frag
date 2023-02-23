#version 330

in vec2 fragment_canvas_position;

out vec4 color;

uniform sampler2D canvas;

void main() {
    color = texture(canvas, fragment_canvas_position);
    //vec4 c = texture(canvas, fragment_canvas_position);
    //float depth = (c.a + 1.0) / 2.0;
    //color = vec4(depth, depth, depth, depth);
}