#version 330

in vec3 position;
in vec2 texture_coordinates;

out vec2 fragment_texture_coordinates;
out float depth;

uniform mat4 transform;

void main() {
    fragment_texture_coordinates = texture_coordinates;

    vec4 position = transform * vec4(position.x, position.y, position.z, 1.0);
    depth = position.z; 

    gl_Position = position;
}