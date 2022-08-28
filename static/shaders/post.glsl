#include <common.glsl>

uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;

varying vec2 v_quad_pos;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;
void main() {
    v_quad_pos = a_pos;
    // vec3 pos = u_projection_matrix * u_view_matrix * vec3(a_pos, 1.0);
    vec3 pos = vec3(a_pos, 1.0);
    gl_Position = vec4(pos.xy, 0.0, pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
void main() {
    gl_FragColor = vec4(0.0);
}
#endif
