#version 450 core

struct Material {
    sampler2D diffuse;
    sampler2D specular;
};

out vec4 fragColor;

uniform Material material;
uniform sampler2D u_texture0;
uniform sampler2D u_texture1;

in vec2 TexCoord;

void main() {
    vec3 diffuse = vec3(texture(material.diffuse, TexCoord));
    vec3 specular = vec3(texture(material.specular, TexCoord));

    fragColor = vec4(diffuse + specular, 1.0);
}
