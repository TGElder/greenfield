#version 330

in vec2 fragment_texture_coordinates;
in float depth;

uniform sampler2D tex;

out vec4 color;

void main() {
    color = texture(tex, fragment_texture_coordinates);

    if (color.a < 0.5) {
        discard;
    }
    
    color.a = depth;
}