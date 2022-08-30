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

vec4 invert(vec4 col) {
    vec3 c = vec3(1.0) - col.rgb;
    return vec4(c, col.a);
}

float rand(vec2 co) {
    return fract(sin(dot(co, vec2(12.9898, 78.233))) * 43758.5453);
}

vec2 rotateCW(vec2 p, float a)
{
    mat2 m = mat2(cos(a), -sin(a), sin(a), cos(a));
    return p * m;
}

vec2 torus_normalize(vec2 pos, vec2 world_size) {
    if (pos.y < 0.0) {
        pos.y += world_size.y;
    }
    if (pos.y > world_size.y) {
        pos.y -= world_size.y;
    }
    if (pos.x < 0.0) {
        pos.x += world_size.x;
    }
    if (pos.x > world_size.x) {
        pos.x -= world_size.x;
    }
    return pos;
}

vec2 torus_delta(vec2 a, vec2 b, vec2 world_size) {
    vec2 delta = a - b;
    if (abs(delta.x) > world_size.x / 2.0) {
        delta.x -= world_size.x * sign(delta.x);
    }
    if (abs(delta.y) > world_size.y / 2.0) {
        delta.y -= world_size.y * sign(delta.y);
    }
    return delta;
}

float dist_to_line(vec2 uv, float angle, float period)
{
    uv = rotateCW(uv, angle);
    float leftPeriod = floor(uv.x / period) * period;
    return min(uv.x - leftPeriod, leftPeriod + period - uv.x);
}

// Simplex 2D noise

vec2 hash(vec2 p) // replace this by something better
{
    p = vec2(dot(p,vec2(127.1,311.7)), dot(p,vec2(269.5,183.3)));
    return -1.0 + 2.0*fract(sin(p)*43758.5453123);
}

float noise( in vec2 p )
{
    const float K1 = 0.366025404; // (sqrt(3)-1)/2;
    const float K2 = 0.211324865; // (3-sqrt(3))/6;

    vec2  i = floor( p + (p.x+p.y)*K1 );
    vec2  a = p - i + (i.x+i.y)*K2;
    float m = step(a.y,a.x); 
    vec2  o = vec2(m,1.0-m);
    vec2  b = a - o + K2;
    vec2  c = a - 1.0 + 2.0*K2;
    vec3  h = max( 0.5-vec3(dot(a,a), dot(b,b), dot(c,c) ), 0.0 );
    vec3  n = h*h*h*h*vec3( dot(a,hash(i+0.0)), dot(b,hash(i+o)), dot(c,hash(i+1.0)));
    return dot( n, vec3(70.0) );
}
