use crate::{
    Registry,
    components::{
        CModelNode,
        CTransform,
        CModelMatrix,
    },
    gfx::{
        ElementArrayMesh,
        mesh::BufferUsage,
    },
    VersionedIndex,
    resources::{
        Shader,
        shader::UniformVariable
    },
    utils::to_abs_path
};

use super::{
    mvp_matrices::s_set_model_matrix,
    draw::s_draw_entity
};

pub struct Grid {
    shader: VersionedIndex
}

pub fn s_create_grid(
    registry: &mut Registry
) -> Result<Grid, Box<dyn std::error::Error>>{
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
        .create_vbo_3_f32(
            0,
            vertices.len(),
            Some(vertices)
        )?;

    build_axis(registry, mesh, glm::vec3(0.0, 0.0, 0.0), glm::vec3(0.0, 0.0, 0.0))?;

    let shader = registry.create_resource(Shader::new(
        &to_abs_path("assets/shaders/grid")?,
        vec![
            UniformVariable::ProjectionMatrix("projection".to_string()),
            UniformVariable::ViewMatrix("view".to_string()),
            UniformVariable::NearPlane("near".to_string()),
            UniformVariable::FarPlane("far".to_string())
        ]
    )?)?;

    Ok(Grid { shader })
}

fn build_axis(
    registry: &mut Registry,
    mesh: ElementArrayMesh,
    translate: glm::Vec3,
    scale: glm::Vec3
) -> Result<(), Box<dyn std::error::Error>> {
    let grid = registry.create_entity("grid")?
        .with(CModelNode {
            mesh: Some(mesh),
            ..CModelNode::default()
        })?
        .with(CTransform {
            translate,
            scale: Some(scale),
            ..CTransform::default()
        })?
        .with(CModelMatrix::default())?
        .done()?;
    s_set_model_matrix(&grid, registry);

    Ok(())
}

pub fn s_draw_grid(
    registry: &Registry,
    camera: &VersionedIndex,
    grid: &Grid
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(shader) = registry.get_resource::<Shader>(&grid.shader) {
        let grid = registry.get_entities_by_tag("grid");

        for line in grid {
            s_draw_entity(
                &line,
                registry,
                camera,
                shader,
                super::draw::DrawMode::Triangles
            );
        }
    }

    Ok(())
}
