#include <common.glsl>

#ifdef VERTEX_SHADER
varying vec2 v_quad_pos;
attribute vec2 a_pos;
uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;
void main() {
    v_quad_pos = a_pos;
    vec3 pos = u_projection_matrix * u_view_matrix * vec3(a_pos, 1.0);
    gl_Position = vec4(a_pos.xy, 0.0, pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
uniform vec2 u_size;
uniform vec2 u_offset;
varying vec2 v_quad_pos;

uniform vec3 u_color_1;
uniform vec3 u_color_2;
uniform float cellSize;

ivec2 cellIndex(vec2 uv)
{
    return ivec2(floor(uv.x), floor(uv.y));
}

vec2 cellCenter(ivec2 iuv)
{
    return vec2(iuv);
}

float distToLine(vec2 uv, float angle, float period)
{
    // uv.x += u_time;
    uv = rotateCW(uv, angle);
    float leftPeriod = floor(uv.x / period) * period;
    return min(uv.x - leftPeriod, leftPeriod + period - uv.x);
}

vec4 renderLine(vec2 uv, float clampBrightness, float width, float drop, float rotSpeed, float angleRange, float spreadness)
{
    uv += vec2(sin(u_time), cos(u_time)) * u_size.x / cellSize;
    return vec4(clampBrightness * smoothstep(drop, width, distToLine(uv, sin((u_time + 1000.0) * rotSpeed) * angleRange, spreadness)));
}

vec4 renderCell(vec2 uv)
{
    ivec2 iuv = cellIndex(uv);
    vec2 cuv = cellCenter(iuv);

    vec3 color1 = u_color_1;
    vec3 color2 = u_color_2;

    vec4 col = vec4(mix(color1, color2, float(mod(iuv.x + iuv.y, 2) == 0)), 1.0);
    // float dist = distance(cuv, vec2(cos(u_time), sin(u_time * 1.3)) * 900.0 + vec2(350.0)) / cellSize / 35.0;
    // dist = clamp(dist, 0.0, 1.0);
    // col = alphaBlend(col, vec4(0.3 * smoothstep(0.8, 0.0, dist)));
    float clampBrightness = 0.15;
    col = alphaBlend(col, renderLine(cuv, clampBrightness, 5.0, 12.0, 0.006, 33.0, 28.0));
    col = alphaBlend(col, renderLine(cuv, clampBrightness, 3.0, 5.0, 0.003, 60.0, 40.0));
    col = alphaBlend(col, renderLine(cuv, clampBrightness, 4.0, 2.0, 0.006, 60.0, 50.0));
    col = alphaBlend(col, renderLine(cuv, clampBrightness, 7.0, 12.0, 0.002, 180.0, 70.0));
    return col;
}

void main() {
    vec2 uv = u_offset + (v_quad_pos * u_size + u_size / 2.0) / cellSize;
    vec4 col = renderCell(uv);
    gl_FragColor = col;
}
#endif
