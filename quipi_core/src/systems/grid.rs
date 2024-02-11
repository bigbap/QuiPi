use uuid::Uuid;

use crate::{
    components::{
        CModelMatrix,
        CModelNode,
        CTransform,
        CDrawable,
        CTag,
        CName
    },
    wrappers::opengl::{
        draw::DrawMode,
        buffer::BufferUsage,
    },
    resources::{
        RShader,
        shader::UniformVariable
    },
    systems::rendering::*,
    utils::to_abs_path,
    Registry,
    VersionedIndex
};

use self::mesh::{ElementArrayMesh, ShaderLocation};

use super::rendering::draw::draw_entity;

const GRID_TAG: &str = "quipi_grid_74872346";

pub struct Grid {}

impl Grid {
    pub fn new(
        registry: &mut Registry,
        camera: VersionedIndex
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

        let r_shader = RShader::new(
            &to_abs_path("assets/shaders/grid")?,
            vec![
                UniformVariable::ProjectionMatrix("projection".to_string()),
                UniformVariable::ViewMatrix("view".to_string()),
                UniformVariable::NearPlane("near".to_string()),
                UniformVariable::FarPlane("far".to_string())
            ]
        )?;

        let id = Uuid::new_v4().to_string();

        let shader = registry.resources.create()?;
        registry.resources.add(&shader, CName { name: id });
        registry.resources.add(&shader, r_shader);

        build_axis(
            registry,
            shader,
            camera,
            mesh,
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(0.0, 0.0, 0.0)
        )?;

        Ok(Self {})
    }

    pub fn draw(
        &self,
        registry: &mut Registry,
        camera: &VersionedIndex
    ) -> Result<(), Box<dyn std::error::Error>> {
        let grid = registry.entities.query::<CTag>(CTag { tag: GRID_TAG.to_string() });

        for line in grid {
            if let Some(drawable) = registry.entities.get::<CDrawable>(&line) {
                let shader = drawable.shader;
                draw_entity(
                    &line,
                    registry,
                    camera,
                    &shader,
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
    camera: VersionedIndex,
    mesh: ElementArrayMesh,
    translate: glm::Vec3,
    scale: glm::Vec3
) -> Result<(), Box<dyn std::error::Error>> {
    let transform = CTransform {
        translate,
        scale,
        ..CTransform::default()
    };
    let model_matrix = CModelMatrix(transform.to_matrix());
    let mesh = CModelNode {
        mesh: Some(mesh),
        ..CModelNode::default()
    };

    let entity = registry.entities.create()?;
    registry.entities.add(&entity, CTag { tag: GRID_TAG.to_string() });
    registry.entities.add(&entity, mesh);
    registry.entities.add(&entity, CDrawable { shader, camera, textures: vec![], active: true });
    registry.entities.add(&entity, transform);
    registry.entities.add(&entity, model_matrix);

    Ok(())
}

