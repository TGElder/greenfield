#version 330

in vec3 position;
in vec3 color;

out vec3 fragment_color;

uniform mat4 transform;

void main() {
    fragment_color = color;
    gl_Position = transform * vec4(position.x, position.y, position.z, 1.0);
}