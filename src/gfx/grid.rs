use crate::prelude::ecs::{CDrawable, CMesh, CModelMatrix, CTransform};
use crate::{
    components::{CName, CTag},
    resources::{shader::UniformVariable, RShader},
    systems::rendering::mesh::{ElementArrayMesh, ShaderLocation},
    utils::to_abs_path,
    wrappers::opengl::{buffer::BufferUsage, draw::DrawMode},
    GlobalRegistry, Index, QPResult,
};

use super::draw::draw_entity;

const GRID_TAG: &str = "quipi_grid_74872346";

pub struct Grid {}

impl Grid {
    pub fn new(registry: &mut GlobalRegistry, camera: Index) -> QPResult<Self> {
        let indices = &[0, 1, 2, 2, 3, 0];
        let vertices = &[
            -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0,
        ];

        let mut mesh = ElementArrayMesh::new(6, BufferUsage::StaticDraw)?;
        mesh.with_ebo(indices)?
            .with_vbo::<3, f32>(ShaderLocation::Zero, vertices)?;

        let r_shader = RShader::new(
            &to_abs_path("assets/shaders/grid")?,
            vec![
                UniformVariable::ProjectionMatrix("projection".to_string()),
                UniformVariable::ViewMatrix("view".to_string()),
                UniformVariable::NearPlane("near".to_string()),
                UniformVariable::FarPlane("far".to_string()),
            ],
        )?;

        let shader = registry.resources.create()?;
        registry.resources.add(
            &shader,
            CName {
                name: "grid_3d".to_string(),
            },
        );
        registry.resources.add(&shader, r_shader);

        build_axis(
            registry,
            shader,
            camera,
            mesh,
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(0.0, 0.0, 0.0),
        )?;

        Ok(Self {})
    }

    pub fn draw(&self, registry: &mut GlobalRegistry, camera: &Index) -> QPResult<()> {
        let grid = registry.entity_manager.query::<CTag>(CTag {
            tag: GRID_TAG.to_string(),
        });

        for line in grid {
            if let Some(drawable) = registry.entity_manager.get::<CDrawable>(&line) {
                let shader = drawable.shader;
                draw_entity(&line, registry, camera, &shader, DrawMode::Triangles);
            }
        }

        Ok(())
    }
}

fn build_axis(
    registry: &mut GlobalRegistry,
    shader: Index,
    camera: Index,
    mesh: ElementArrayMesh,
    translate: glm::Vec3,
    scale: glm::Vec3,
) -> QPResult<()> {
    let transform = CTransform {
        translate,
        scale,
        ..CTransform::default()
    };
    let model_matrix = CModelMatrix(transform.to_matrix());
    let mesh = CMesh {
        mesh: Some(mesh),
        ..CMesh::default()
    };

    let entity = registry.entity_manager.create()?;
    registry.entity_manager.add(
        &entity,
        CTag {
            tag: GRID_TAG.to_string(),
        },
    );
    registry.entity_manager.add(&entity, mesh);
    registry.entity_manager.add(
        &entity,
        CDrawable {
            shader,
            camera,
            textures: vec![],
            active: true,
        },
    );
    registry.entity_manager.add(&entity, transform);
    registry.entity_manager.add(&entity, model_matrix);

    Ok(())
}

// inspiration from https://asliceofrendering.com/scene%20helper/2020/01/05/InfiniteGrid/
const VERT_SHADER: &str = r#"
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
"#;

// copied from https://github.com/ArjunNair/egui_sdl2_gl/tree/main
const FRAG_SHADER: &str = r#"
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
"#;
