#version 450 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec2 aTexCoord;

uniform mat4 mvpMatrix;

out vec4 color;
out vec2 texCoord;

void main(){
    gl_Position = mvpMatrix * vec4(aPos, 1.0);

    color = aColor;
    texCoord = aTexCoord;
}
