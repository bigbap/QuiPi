use uuid::Uuid;

use crate::{
    components::{
        CModelMatrix,
        CModelNode,
        CTransform, CShader, CTag, CName
    },
    wrappers::opengl::{
        draw::DrawMode,
        buffer::BufferUsage,
    },
    resources::{
        Shader,
        shader::UniformVariable
    },
    systems::rendering::*,
    utils::to_abs_path,
    Registry,
    VersionedIndex
};

use self::mesh::{ElementArrayMesh, ShaderLocation};

use super::rendering::draw::s_draw_entity;

const GRID_TAG: &str = "quipi_grid_74872346";

pub struct Grid {}

impl Grid {
    pub fn new(
        registry: &mut Registry,
    ) -> Result<Self, Box<dyn std::error::Error>>{
        let indices = &[0, 1, 2, 2, 3, 0];
        let vertices = &[
            -1.0, -1.0, 0.0,
            1.0, -1.0, 0.0,
            1.0, 1.0, 0.0,
            -1.0, 1.0, 0.0,
        ];

        let mut mesh = ElementArrayMesh::new(6, BufferUsage::StaticDraw)?;
        mesh
            .with_ebo(indices)?
            .with_vbo::<3, f32>(
                ShaderLocation::Zero,
                vertices
            )?;

        let shader = Shader::new(
            &to_abs_path("assets/shaders/grid")?,
            vec![
                UniformVariable::ProjectionMatrix("projection".to_string()),
                UniformVariable::ViewMatrix("view".to_string()),
                UniformVariable::NearPlane("near".to_string()),
                UniformVariable::FarPlane("far".to_string())
            ]
        )?;

        let id = Uuid::new_v4().to_string();

        registry.resources.start_create()?;
        registry.resources.add(CName::new(&id, registry));
        registry.resources.add(shader);
        let shader = registry.resources.end_create()?;

        build_axis(
            registry,
            shader,
            mesh,
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(0.0, 0.0, 0.0)
        )?;

        Ok(Self {})
    }

    pub fn draw(
        &self,
        registry: &'static Registry,
        camera: &VersionedIndex
    ) -> Result<(), Box<dyn std::error::Error>> {
        let grid = registry.entities.query::<CTag>(CTag { tag: GRID_TAG.to_string() });

        for line in grid {
            if let Some(shader_id) = registry.entities.get::<CShader>(&line) {
                s_draw_entity(
                    &line,
                    registry,
                    camera,
                    shader_id,
                    DrawMode::Triangles
                );
            }
        }

        Ok(())
    }
}

fn build_axis(
    registry: &mut Registry,
    shader: VersionedIndex,
    mesh: ElementArrayMesh,
    translate: glm::Vec3,
    scale: glm::Vec3
) -> Result<(), Box<dyn std::error::Error>> {
    let transform = CTransform {
        translate,
        scale: Some(scale),
        ..CTransform::default()
    };
    let model_matrix = CModelMatrix(transform.to_matrix());
    let mesh = CModelNode {
        mesh: Some(mesh),
        ..CModelNode::default()
    };

    registry.entities.start_create()?;
    registry.entities.add(CTag { tag: GRID_TAG.to_string() });
    registry.entities.add(mesh);
    registry.entities.add(CShader { shader });
    registry.entities.add(transform);
    registry.entities.add(model_matrix);
    registry.entities.end_create()?;


    Ok(())
}

