#version 330

in vec3 fragment_color;
flat in float id_in_float;

out vec4 color;

void main() {
    color = vec4(fragment_color, 1.0);
}