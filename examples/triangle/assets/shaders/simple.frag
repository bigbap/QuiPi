#version 450 core

vec4 lightColor = vec4(0.33, 0.42, 0.18, 1.0);

in vec4 vertexColor;
in vec2 vertexCoord;

out vec4 fragColor;

uniform sampler2D texture1;

void main(){
    // fragColor = vertexColor;
    fragColor = lightColor * texture(texture1, vertexCoord);
}
