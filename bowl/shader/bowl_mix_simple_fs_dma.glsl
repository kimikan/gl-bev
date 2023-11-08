
#version 320 es
#extension GL_OES_EGL_image_external_essl3 : require
precision highp float;
uniform samplerExternalOES aTexture0;
in vec2 vTexcoord0;
layout(location = 0) out vec4 outColor;
void main() {
    outColor.a = 1.0;
    float sw = 1920;
    float x = floor(vTexcoord0.x * sw + 0.1);
    float alpha = floor(mod(x, 2.0) + 0.1);
    vec2 uv0 = vec2(floor(x - (1.0 - alpha) * 0.5) / sw, vTexcoord0.y);
    vec4 uyvy = texture(aTexture0, uv0);
    vec3 temp = uyvy.grb * vec3(alpha) + uyvy.arb * (1.0 - alpha);
    vec3 c = vec3(temp.r-0.062745)*1.164;
    c += (temp.b - 0.5) * vec3(1.596, -0.813, 0.0);
    c += (temp.g - 0.5) * vec3(0.0, -0.392, 2.017);
    outColor.rgb = c;
}