#version 330

in vec2 fragment_texture_coordinates;
in float depth;

out vec4 color;

uniform sampler2D tex;

void main() {
    color = texture(tex, fragment_texture_coordinates);
    color.a = depth;
}