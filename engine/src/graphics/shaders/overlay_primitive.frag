#version 330

in vec2 fragment_texture_coordinates;
in float depth;
in float shade;

uniform sampler2D base;
uniform sampler2D overlay;

out vec4 color;

void main() {
    vec4 base_color = texture(base, fragment_texture_coordinates);
    vec4 overlay_color = texture(overlay, fragment_texture_coordinates);
    float alpha = overlay_color.a;
    color = pow(base_color * (1.0 - alpha) * shade + overlay_color * alpha, vec4(1.0 / 2.0));
    color.a = depth;
}