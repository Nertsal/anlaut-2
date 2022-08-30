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
uniform sampler2D u_frame_texture;
uniform ivec2 u_frame_texture_size;

uniform vec2 u_world_size;

void main() {
    vec2 texture_size = vec2(u_frame_texture_size);
    vec2 coord = gl_FragCoord.xy / texture_size;
    vec4 col = texture2D(u_frame_texture, coord);

    // Transform screen coordinates into world coordinates
    vec3 pos3 = inverse(u_projection_matrix * u_view_matrix) * vec3(v_quad_pos, 1.0);
    vec2 pos = pos3.xy / pos3.z;
    pos = torus_normalize(pos, u_world_size);
    vec2 delta = torus_delta(pos, vec2(0.0), u_world_size);

    // Actual stuff
    float len = length(delta);
    len += noise(delta * sin(u_time));
    if (len < 5.0) {
        col = invert(col);
    }

    gl_FragColor = col;
}
#endif
