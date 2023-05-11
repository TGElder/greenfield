#version 330

in vec2 fragment_texture_coordinates;
in float depth;

out vec4 color;

uniform sampler2D base;
uniform sampler2D overlay;

void main() {
    vec4 base_color = texture(base, fragment_texture_coordinates);
    vec4 overlay_color = texture(overlay, fragment_texture_coordinates);
    float alpha = overlay_color.a;
    color = base_color * (1.0 - alpha) + overlay_color * alpha;
    color.a = depth;
}