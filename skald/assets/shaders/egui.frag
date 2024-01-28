// copied from https://github.com/ArjunNair/egui_sdl2_gl/tree/main

#version 450 core

in vec4 color;
in vec2 uvCoords;

uniform sampler2D u_sampler;

out vec4 fragColor;

// 0-255 sRGB  from  0-1 linear
vec3 srgbFromLinear(vec3 rgb) {
    bvec3 cutoff = lessThan(rgb, vec3(0.0031308));
    vec3 lower = rgb * vec3(3294.6);
    vec3 higher = vec3(269.025) * pow(rgb, vec3(1.0 / 2.4)) - vec3(14.025);
    return mix(higher, lower, vec3(cutoff));
}

vec4 srgbaFromLinear(vec4 rgba) {
    return vec4(srgbFromLinear(rgba.rgb), 255.0 * rgba.a);
}

vec3 linearFromSrgb(vec3 srgb) {
    bvec3 cutoff = lessThan(srgb, vec3(10.31475));
    vec3 lower = srgb / vec3(3294.6);
    vec3 higher = pow((srgb + vec3(14.025)) / vec3(269.025), vec3(2.4));
    return mix(higher, lower, vec3(cutoff));
}

vec4 linearFromSrgba(vec4 srgba) {
    return vec4(linearFromSrgb(srgba.rgb), srgba.a / 255.0);
}

void main() {
    // vec4 textureRgba = linearFromSrgba(texture(u_sampler, uvCoords) * 255.0);
    // fragColor = color * textureRgba;

    fragColor = color;
    // fragColor = vec4(color.w, color.w, color.w, 1.0);
}
