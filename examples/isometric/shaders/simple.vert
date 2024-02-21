#version 450 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoords;
layout (location = 2) in vec3 aColor;

uniform mat4 mvpMatrix;

out vec3 pos;
out vec2 texCoords;
out vec3 color;

void main(){
    gl_Position = mvpMatrix * vec4(aPos, 1.0);
    
    pos = aPos;
    texCoords = aTexCoords;
    color = aColor;
}
