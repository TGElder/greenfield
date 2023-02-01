#version 330

in vec3 position;
in vec3 color;

out vec3 fragment_color;
out float depth;

uniform mat4 transform;

void main() {
    fragment_color = color;

    vec4 position = transform * vec4(position.x, position.y, position.z, 1.0);
    depth = position.z; 

    gl_Position = position;
}