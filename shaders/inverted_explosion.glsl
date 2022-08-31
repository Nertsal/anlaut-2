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
    // vec3 pos = vec3(a_pos, 1.0);
    gl_Position = vec4(pos.xy, 0.0, pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
uniform sampler2D u_frame_texture;
uniform ivec2 u_frame_texture_size;

void main() {
    // Get texture pixel
    vec2 texture_size = vec2(u_frame_texture_size);
    vec2 coord = gl_FragCoord.xy / texture_size;
    vec4 col = texture2D(u_frame_texture, coord);

    // Actual stuff
    float len = length(v_quad_pos);
    len += (1.0 - noise(v_quad_pos * sin(u_time))) / 5.0;
    if (len > 1.0) {
        discard;
    }

    col = invert(col);
    gl_FragColor = col;
}
#endif
