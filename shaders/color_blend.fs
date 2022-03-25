#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
uniform vec4 blend_color;
void main() {
    vec4 tex_color = texture(tex, v_tex_coords);
    float blend_value = 0.5;
    color = blend_value * blend_color +  (1 - blend_value) * tex_color;
}