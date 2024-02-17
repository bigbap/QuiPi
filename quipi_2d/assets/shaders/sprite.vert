#version 450 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec2 aTexCoords;
layout (location = 3) in float aTexIndex;

uniform mat4 view;
uniform mat4 projection;

out vec4 color;
out vec2 texCoords;
out float texIndex;

void main(){
    gl_Position = vec4(aPos, 1.0);

    color = aColor;
    texCoords = aTexCoords;
    texIndex = aTexIndex;
}
