#version 330

in vec3 position;
in vec2 offset;
in vec2 texture_coordinates;

out vec2 fragment_texture_coordinates;
out float depth;

uniform mat4 transform;
uniform mat4 scale;

void main() {
    fragment_texture_coordinates = texture_coordinates;

    vec4 center = transform * vec4(position.x, position.y, position.z, 1.0);
    depth = center.z; 

    vec4 screen_offset = scale * vec4(offset.x, offset.y, -offset.y, 0.0);

    gl_Position = center + screen_offset;
}