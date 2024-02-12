#version 450 core

in vec4 color;
in vec2 texCoord;

uniform sampler2D u_texture;

out vec4 fragColor;

void main() {
    fragColor = color * texture(u_texture, texCoord);
}
