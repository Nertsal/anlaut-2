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
uniform vec2 u_size;
uniform float cellSize;

uniform vec3 u_color_1;
uniform vec3 u_color_2;

ivec2 cellIndex(vec2 uv)
{
    return ivec2(floor(uv.x), floor(uv.y));
}

vec2 cellCenter(ivec2 iuv)
{
    return vec2(iuv);
}

vec4 renderLine(vec2 uv, float clampBrightness, float width, float drop, float rotSpeed, float angleRange, float spreadness)
{
    uv += vec2(sin(u_time), cos(u_time)) * u_size.x / cellSize;
    float angle = sin((u_time + 1000.0) * rotSpeed) * angleRange;
    float distance = dist_to_line(uv, angle, spreadness);
    float value = smoothstep(drop, width, distance);
    return vec4(clampBrightness * value);
}

vec4 renderCell(vec2 uv)
{
    ivec2 iuv = cellIndex(uv);
    vec2 cuv = cellCenter(iuv);

    vec3 color1 = u_color_1;
    vec3 color2 = u_color_2;

    vec4 col = vec4(mix(color1, color2, float(mod(iuv.x + iuv.y, 2) == 0)), 1.0);
    float clampBrightness = 0.15;
    col = alphaBlend(col, renderLine(cuv, clampBrightness, 5.0, 12.0, 0.006, 33.0, 18.0));
    col = alphaBlend(col, renderLine(cuv, clampBrightness, 3.0, 5.0, 0.003, 60.0, 20.0));
    col = alphaBlend(col, renderLine(cuv, clampBrightness, 4.0, 2.0, 0.006, 60.0, 30.0));
    col = alphaBlend(col, renderLine(cuv, clampBrightness, 7.0, 12.0, 0.002, 180.0, 40.0));
    return col;
}

void main() {
    // Transform screen coordinates into world coordinates
    vec3 pos = inverse(u_projection_matrix * u_view_matrix) * vec3(v_quad_pos, 1.0);
    // Transform into cell coordinates
    vec2 uv = pos.xy / cellSize;
    vec4 col = renderCell(uv);
    gl_FragColor = col;
}
#endif
