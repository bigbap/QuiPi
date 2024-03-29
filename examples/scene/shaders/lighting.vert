#version 450 core

layout (location = 0) in vec3 aPos;
layout (location = 2) in vec2 aTexCoords;
layout (location = 3) in vec3 aNormal;

uniform mat4 model;
uniform mat4 mvpMatrix;

out vec3 FragPos;
out vec3 Normal;
out vec2 TexCoords;

void main(){
    gl_Position = mvpMatrix * vec4(aPos, 1.0);

    FragPos = vec3(model * vec4(aPos, 1.0));
    Normal = mat3(transpose(inverse(model))) * aNormal;
    TexCoords = aTexCoords;
}
