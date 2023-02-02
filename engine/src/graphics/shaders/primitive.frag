#version 330

in vec3 fragment_color;
in float depth;

out vec4 color;

void main() {
    color = vec4(fragment_color, depth);
}