#include <common.glsl>

uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;
uniform mat3 u_model_matrix;

varying vec2 v_quad_pos;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;
void main() {
    v_quad_pos = a_pos;
    vec3 pos = u_projection_matrix * u_view_matrix * u_model_matrix * vec3(a_pos, 1.0);
    // vec3 pos = vec3(v_quad_pos, 1.0);
    gl_Position = vec4(pos.xy, 0.0, pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
uniform vec4 u_color;
uniform float u_offset;
uniform float u_angle;
uniform float u_width;
uniform float u_spacing;

void main() {
    // vec3 pos3 = inverse(u_projection_matrix * u_view_matrix * u_model_matrix) * vec3(v_quad_pos, 1.0);
    // vec2 pos = pos3.xy / pos3.z;
    vec2 pos = v_quad_pos;
    float dist = dist_to_line(pos, u_angle, u_spacing);
    if (dist > u_width) {
        discard;
    }
    gl_FragColor = u_color;
}
#endif
