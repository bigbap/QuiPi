#version 450 core

in vec4 color;
in vec2 texCoords;
in float texIndex;

uniform sampler2D[8] u_textures;

out vec4 fragColor;

void main() {
    int texId = int(texIndex);

    if (texId == 0) {
        fragColor = color;
    } else {
        fragColor = texture(u_textures[texId - 1], texCoords);
    }
}
