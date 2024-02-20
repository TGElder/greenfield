#version 330

const float PI = 3.1415927;

in vec3 position;
in vec3 normal;
in vec3 color;

out vec3 fragment_color;
out float depth;
out float shade;

uniform mat4 transform;
uniform vec3 light_direction;

void main() {
    fragment_color = color;

    vec4 position = transform * vec4(position.x, position.y, position.z, 1.0);
    depth = position.z;

    float angle = acos(dot(normal, light_direction));
    shade = angle / PI;
    gl_Position = position;
}