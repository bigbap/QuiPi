// inspiration from https://asliceofrendering.com/scene%20helper/2020/01/05/InfiniteGrid/

#version 450 core

layout (location = 0) in vec3 aPos;

uniform mat4 view;
uniform mat4 projection;

out mat4 fragView;
out mat4 fragProj;
out vec3 nearPoint;
out vec3 farPoint;

vec3 UnprojectedPoint(float x, float y, float z, mat4 view, mat4 projection) {
    mat4 viewInv = inverse(view);
    mat4 projInv = inverse(projection);
    vec4 unprojectedPoint = viewInv * projInv * vec4(x, y, z, 1.0);

    return unprojectedPoint.xyz / unprojectedPoint.w;
}

void main(){
    vec3 p = aPos.xyz;
    fragView = view;
    fragProj = projection;
    nearPoint = UnprojectedPoint(p.x, p.y, 0.0, view, projection).xyz;
    farPoint = UnprojectedPoint(p.x, p.y, 1.0, view, projection).xyz;

    gl_Position = vec4(p, 1.0);
}