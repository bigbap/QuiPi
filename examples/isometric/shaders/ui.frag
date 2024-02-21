#version 450 core

in vec3 ui_fragPos;
in vec4 ui_fragColor;

out vec4 ui_outColor;

void main(){
    ui_outColor = ui_fragColor;
}
