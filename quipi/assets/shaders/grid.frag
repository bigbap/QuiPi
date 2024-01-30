// inspiration from https://asliceofrendering.com/scene%20helper/2020/01/05/InfiniteGrid/

#version 450 core

uniform float near;
uniform float far;

in mat4 fragView;
in mat4 fragProj;
in vec3 nearPoint;
in vec3 farPoint;

out vec4 fragColor;

vec4 grid(vec3 fragPos3D, float scale, bool drawAxis) {
    vec2 coord = fragPos3D.xz * scale;
    vec2 derivative = fwidth(coord);
    vec2 grid = abs(fract(coord - 0.5) - 0.5) / derivative;
    float line = min(grid.x, grid.y);
    float minZ = min(derivative.y, 1);
    float minX = min(derivative.x, 1);
    vec4 color = vec4(0.2, 0.2, 0.2, 1.0 - min(line, 1.0));

    if (fragPos3D.x > -0.1 * minX && fragPos3D.x < 0.1 * minX) {
        color.z = 1.0;
    }

    if (fragPos3D.z > -0.1 * minZ && fragPos3D.z < 0.1 * minZ) {
        color.x = 1.0;
    }

    return color;
}

float computeDepth(vec3 pos) {
    vec4 clipSpacePos = fragProj * fragView * vec4(pos.xyz, 1.0);
    
    return (clipSpacePos.z / clipSpacePos.w);
}

float computeLinearDepth(vec3 pos) {
    vec4 clipSpacePos = fragProj * fragView * vec4(pos.xyz, 1.0);
    float clipSpaceDepth = (clipSpacePos.z / clipSpacePos.w) * 2.0 - 1.0;
    float linearDepth = (2.0 * near * far) / (far + near - clipSpaceDepth * (far - near));

    return linearDepth / far;
}

void main() {
    float t = -nearPoint.y / (farPoint.y - nearPoint.y);
    vec3 fragPos3D = nearPoint + t * (farPoint - nearPoint);

    gl_FragDepth = computeDepth(fragPos3D);

    float linearDepth = computeLinearDepth(fragPos3D);
    float fading = max(0, (0.5 - linearDepth));

    fragColor = (grid(fragPos3D, 10, true) + grid(fragPos3D, 1, true)) * float(t > 0);
    fragColor.a *= fading;
}
