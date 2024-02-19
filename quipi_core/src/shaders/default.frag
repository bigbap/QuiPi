#version 450 core

in vec4 color;
in vec2 texCoords;
in float texIndex;

uniform sampler2D u_textures[16];

out vec4 fragColor;

void main() {
    int texId = int(texIndex);

    fragColor = color * texture(u_textures[texId], texCoords);
}
