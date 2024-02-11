#version 450 core

layout (location = 0) in vec2 aPos;
layout (location = 1) in vec4 aColor;

uniform mat4 mvpMatrix;

out vec4 color;

void main(){
    gl_Position = mvpMatrix * vec4(aPos, 0.0, 1.0);

    color = aColor;
}
