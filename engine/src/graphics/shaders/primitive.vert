#version 330

in vec3 position;
in vec3 color;
in uint id;

out vec3 fragment_color;
flat out float id_in_float;

uniform mat4 transform;

void main() {
    fragment_color = color;
    id_in_float = uintBitsToFloat(id);

    gl_Position = transform * vec4(position.x, position.y, position.z, 1.0);
}