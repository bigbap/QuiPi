// copied from https://github.com/ArjunNair/egui_sdl2_gl/tree/main

#version 450 core

layout (location = 0) in vec2 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec2 aUVCoords; 

uniform vec2 u_screenSize;
uniform mat4 u_mvpMatrix;

out vec4 color;
out vec2 uvCoords;

// 0-1 linear  from  0-255 sRGB
vec3 linearFromSrgb(vec3 srgb) {
    bvec3 cutoff = lessThan(srgb, vec3(10.31475));
    vec3 lower = srgb / vec3(3294.6);
    vec3 higher = pow((srgb + vec3(14.025)) / vec3(269.025), vec3(2.4));
    return mix(higher, lower, cutoff);
}

vec4 linearFromSrgba(vec4 srgba) {
    return vec4(linearFromSrgb(srgba.rgb), srgba.a / 255.0);
}

void main(){
    gl_Position = u_mvpMatrix * vec4(aPos, 0.0, 1.0);

    color = linearFromSrgba(aColor);
    // color = aColor;
    uvCoords = aUVCoords;
}
