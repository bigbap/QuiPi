#version 450 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec3 color;

void main(){
    // gl_Position = projection * view * model * vec4(aPos, 1.0);
    // gl_Position = view * model * vec4(aPos, 1.0);

    vec4 full = projection * view * model * vec4(aPos, 1.0);
    gl_Position = vec4(0.0, full.y, full.z, 1.0);

    color = aColor;
}
