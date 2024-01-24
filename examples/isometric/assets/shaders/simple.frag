#version 450 core

in vec3 pos;
in vec2 texCoords;
in vec3 color;

uniform sampler2D uTexture;

out vec4 fragColor;

void main(){
    fragColor = texture(uTexture, texCoords);
    
    // gl_FragDepth = pos.y;
    // fragColor = vec4(color, 1.0);
}
