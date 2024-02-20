#version 330

in vec3 fragment_color;
in float depth;
in float shade;

out vec4 color;

void main() {
    color = vec4(fragment_color * shade, depth);
}