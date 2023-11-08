
#version 320 es
precision highp float;
uniform sampler2D aTexture0;
uniform sampler2D aTexture1;
in vec2 vTexcoord0;
in vec2 vTexcoord1;
in vec2 vDoubleAlpha;
layout(location = 0) out vec4 outColor;
void main() {
    outColor.a = 1.0;

    float sw = 1920;
    float x = floor(vTexcoord0.x * sw + 0.1);
    float x2 = floor(vTexcoord1.x * sw + 0.1);
    float alpha = floor(mod(x, 2.0) + 0.1);

    vec4 uyvy0 = texture(aTexture0, vec2(floor(x - (1.0 - alpha) * 0.5) / sw, vTexcoord0.y));
    vec3 temp = uyvy0.grb * vec3(alpha) + uyvy0.arb * (1.0 - alpha);
    vec3 c0 = vec3(temp.r-0.062745)*1.164;
    c0 += (temp.b - 0.5) * vec3(1.596, -0.813, 0.0);
    c0 += (temp.g - 0.5) * vec3(0.0, -0.392, 2.017);

    vec4 uyvy1 = texture(aTexture1, vec2(floor(x2 - (1.0 - alpha) * 0.5) / sw, vTexcoord1.y));
    temp = uyvy1.grb * vec3(alpha) + uyvy1.arb * (1.0 - alpha);
    vec3 c1 = vec3(temp.r-0.062745)*1.164;
    c1 += (temp.b - 0.5) * vec3(1.596, -0.813, 0.0);
    c1 += (temp.g - 0.5) * vec3(0.0, -0.392, 2.017);
    outColor.rgb = c0  * vDoubleAlpha.x + c1 * vDoubleAlpha.y;

}