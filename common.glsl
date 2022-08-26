const float pi = 3.14159;

uniform float u_time;

vec4 alphaBlend(vec4 c1, vec4 c2)
{
    return vec4(
        mix(c1.rgb, c2.rgb, c2.a),
        clamp(max(c1.a, c2.a) + c1.a * c2.a, 0., 1.));
}

int mod(int a, int b)
{
    return a - (b * int(floor(float(a)/float(b))));
}

vec2 N22(vec2 p) 
{
  vec3 a = fract(p.xyx*vec3(123.34, 234.34, 345.65));
  a += dot(a, a+34.45);
  return fract(vec2(a.x*a.y, a.y*a.z));
}

vec2 rotateCW(vec2 p, float a)
{
    mat2 m = mat2(cos(a), -sin(a), sin(a), cos(a));
    return p * m;
}
