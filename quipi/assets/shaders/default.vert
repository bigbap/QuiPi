#version 450 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec2 aUVCoords; 

out vec4 color;
out vec2 uvCoords;

void main(){
    gl_Position = vec4(aPos, 1.0);

    color = aColor;
    uvCoords = aUVCoords;
}
