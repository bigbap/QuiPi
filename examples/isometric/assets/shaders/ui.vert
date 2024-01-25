#version 450 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;

uniform mat4 mvpMatrix;

out vec3 ui_fragPos;
out vec4 ui_fragColor;

void main(){
    gl_Position = mvpMatrix * vec4(aPos, 1.0);
    
    ui_fragPos = aPos;
    ui_fragColor = aColor;
}
