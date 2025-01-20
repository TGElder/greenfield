#version 330

const float PI = 3.1415927;

in vec3 position;
in vec3 normal;
in vec3 color;
in mat4 world_matrix;
in mat4 world_normal_matrix;

uniform mat4 transform;
uniform vec3 light_direction;
uniform float ambient_light;

out vec3 fragment_color;
out float depth;

void main() {
    vec4 position = transform * world_matrix * vec4(position.x, position.y, position.z, 1.0);

    vec3 transformed_normal = normalize((world_normal_matrix * vec4(normal, 1.0)).xyz);
    float light_angle = acos(dot(transformed_normal, light_direction));
    float shade = ambient_light + (1.0 - ambient_light) * (light_angle / PI);

    fragment_color = color * shade;
    depth = position.z;
    gl_Position = position;
}