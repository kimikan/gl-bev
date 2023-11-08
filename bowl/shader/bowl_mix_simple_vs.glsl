
#version 320 es
precision highp float;
layout(location = 0) in vec3 aPosition;
layout(location = 4) in vec2 aTexcoord0;
uniform mat4 aMatrixM;
uniform mat4 aMatrixVP;

out vec2 vTexcoord0;

void main() {
    vec4 pos4 = aMatrixVP * aMatrixM * vec4(aPosition.x, aPosition.y, aPosition.z, 1.0);
    vTexcoord0 = aTexcoord0;
    gl_Position = pos4;
}