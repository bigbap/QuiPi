use crate::{
    Registry,
    components::{
        CModelNode,
        CTransform,
        CModelMatrix,
    },
    gfx::ElementArrayMesh,
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
    let length = 100000.0;
    let size = 0.2;

    for i in -1000..1000 {
        if i == 0 { continue; }

        let x_mesh = ElementArrayMesh::new(&[0, 1])?;
        x_mesh
            .create_vbo_at(&[-1.0, 0.0, 0.0, 1.0, 0.0, 0.0], 0, 3)?
            .create_vbo_at(&[0.3, 0.3, 0.3, 0.3, 0.3, 0.3], 1, 3)?;

        let z_mesh = ElementArrayMesh::new(&[0, 1])?;
        z_mesh
            .create_vbo_at(&[0.0, 0.0, -1.0, 0.0, 0.0, 1.0], 0, 3)?
            .create_vbo_at(&[0.3, 0.3, 0.3, 0.3, 0.3, 0.3], 1, 3)?;

        build_axis(registry, x_mesh, glm::vec3(0.0, 0.0, i as f32 * size), glm::vec3(length, 0.0, 0.0))?;
        build_axis(registry, z_mesh, glm::vec3(i as f32 * size, 0.0, 0.0), glm::vec3(0.0, 0.0, length))?;
    }

    let x_mesh = ElementArrayMesh::new(&[0, 1])?;
    x_mesh
        .create_vbo_at(&[-1.0, 0.0, 0.0, 1.0, 0.0, 0.0], 0, 3)?
        .create_vbo_at(&[1.0, 0.0, 0.0, 1.0, 0.0, 0.0], 1, 3)?;

    let y_mesh = ElementArrayMesh::new(&[0, 1])?;
    y_mesh
        .create_vbo_at(&[0.0, -1.0, 0.0, 0.0, 1.0, 0.0], 0, 3)?
        .create_vbo_at(&[0.0, 1.0, 0.0, 0.0, 1.0, 0.0], 1, 3)?;

    let z_mesh = ElementArrayMesh::new(&[0, 1])?;
    z_mesh
        .create_vbo_at(&[0.0, 0.0, -1.0, 0.0, 0.0, 1.0], 0, 3)?
        .create_vbo_at(&[0.0, 0.0, 1.0, 0.0, 0.0, 1.0], 1, 3)?;

    build_axis(registry, x_mesh, glm::vec3(0.0, 0.0, 0.0), glm::vec3(length, 0.0, 0.0))?;
    build_axis(registry, y_mesh, glm::vec3(0.0, 0.0, 0.0), glm::vec3(0.0, length, 0.0))?;
    build_axis(registry, z_mesh, glm::vec3(0.0, 0.0, 0.0), glm::vec3(0.0, 0.0, length))?;
    
    // let mesh = ElementArrayMesh::new(&[0, 1, 2, 2, 3, 0])?;
    // mesh
    //     .create_vbo_at(&[
    //         -1.0, -1.0, 0.0,
    //         1.0, -1.0, 0.0,
    //         1.0, 1.0, 0.0,
    //         -1.0, 1.0, 0.0,
    //         // 1.0, 1.0, 0.0,
    //         // -1.0, -1.0, 0.0,
    //         // -1.0, 1.0, 0.0,
    //         // -1.0, -1.0, 0.0,
    //         // 1.0, 1.0, 0.0,
    //         // 1.0, -1.0, 0.0
    //     ], 0, 3)?;
    // build_axis(registry, mesh, glm::vec3(0.0, 0.0, 0.0), glm::vec3(0.0, 0.0, 0.0))?;

    let shader = registry.create_resource(Shader::new(
        &format!("{}/shaders/grid", asset_path()),
        vec![
            UniformVariable::ModelMatrix("model".to_string()),
            UniformVariable::ViewMatrix("view".to_string())
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
            translate: Some(translate),
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

fn asset_path() -> String {
    to_abs_path("assets").unwrap().to_string_lossy().to_string()
}
