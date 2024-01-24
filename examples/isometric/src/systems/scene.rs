use engine::{
    Registry,
    systems::{load_obj::{
        self,
        ObjectConfig
    }, mvp_matrices::s_set_model_matrix},
    resources::Texture,
    gfx::{texture::*, ElementArrayMesh}, components::{CMaterial, material::MaterialPart, CModelNode, CTransform, CModelMatrix}
};

use crate::{asset_path, WIDTH, HEIGHT};

pub fn s_load_scene(
    registry: &mut Registry
) -> Result<(), Box<dyn std::error::Error>> {
    let (models, _) = load_obj::s_load_obj_file(format!("{}/objects/cube.obj", asset_path()))?;
    let obj_configs = ObjectConfig::from_obj(models)?;
    let texture = Texture {
        id: from_image(&format!("{}/objects/textures/tex.png", asset_path()))?
    };

    let texture = registry.create_resource(texture)?;

    let tile_map = [
        (0, 0) // , (0, 1), (0, 2),
        // (1, 0), (1, 1), (1, 2),
        // (2, 0), (2, 1), (2, 2),
    ];

    for tile in tile_map {
        for config in &obj_configs {
            println!("{:?}", config.texture_coords);
            println!("{:?}", config.indices);

            let (points, colors, indices) = cube();
            // let mesh = ElementArrayMesh::new(&config.indices)?;
            // mesh
            //     .create_vbo_at(&config.points, 0, 3)?
            //     .create_vbo_at(&config.texture_coords, 1, 2)?;
            let mesh = ElementArrayMesh::new(&indices)?;
            mesh
                .create_vbo_at(&points, 0, 3)?
                .create_vbo_at(&colors, 2, 3)?;

            let cube = registry.create_entity("cube")?
                .with(CModelNode {
                    mesh: Some(mesh),
                    ..CModelNode::default()
                })?
                .with(CMaterial {
                    diffuse: MaterialPart::Texture(texture),
                    ..CMaterial::default()
                })?
                .with(CTransform {
                    translate: Some(glm::vec3(
                        // 0.0,
                        // 0.0,
                        0.0 + (128.0 * tile.0 as f32),
                        0.0,
                        200.0 + (128.0 * tile.1 as f32),
                    )), 
                    scale: Some(glm::vec3(64.0, 64.0, 64.0)),
                    ..CTransform::default()
                })?
                .with(CModelMatrix::default())?
                .done()?;

            s_set_model_matrix(&cube, registry);
        }
    }

    Ok(())
}

fn cube() -> (Vec<f32>, Vec<f32>, Vec<u32>) {
    let points = vec![
        -0.5, -0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5,  0.5, -0.5,
        0.5,  0.5, -0.5,
        -0.5,  0.5, -0.5,
        -0.5, -0.5, -0.5,

        -0.5, -0.5,  0.5,
        0.5, -0.5,  0.5,
        0.5,  0.5,  0.5,
        0.5,  0.5,  0.5,
        -0.5,  0.5,  0.5,
        -0.5, -0.5,  0.5,

        -0.5,  0.5,  0.5,
        -0.5,  0.5, -0.5,
        -0.5, -0.5, -0.5,
        -0.5, -0.5, -0.5,
        -0.5, -0.5,  0.5,
        -0.5,  0.5,  0.5,

        0.5,  0.5,  0.5, 
        0.5,  0.5, -0.5, 
        0.5, -0.5, -0.5, 
        0.5, -0.5, -0.5, 
        0.5, -0.5,  0.5, 
        0.5,  0.5,  0.5, 

        -0.5, -0.5, -0.5,
        0.5, -0.5, -0.5,
        0.5, -0.5,  0.5,
        0.5, -0.5,  0.5,
        -0.5, -0.5,  0.5,
        -0.5, -0.5, -0.5,

        -0.5,  0.5, -0.5,
        0.5,  0.5, -0.5,
        0.5,  0.5,  0.5,
        0.5,  0.5,  0.5,
        -0.5,  0.5,  0.5,
        -0.5,  0.5, -0.5,
    ];

    let colors = vec![
        0.2, 0.2, 0.2,
        0.2, 0.2, 0.2,
        0.2, 0.2, 0.2,
        0.2, 0.2, 0.2,
        0.2, 0.2, 0.2,
        0.2, 0.2, 0.2,

        0.3, 0.5, 0.3,
        0.3, 0.5, 0.3,
        0.3, 0.5, 0.3,
        0.3, 0.5, 0.3,
        0.3, 0.5, 0.3,
        0.3, 0.5, 0.3,

        0.8, 0.3, 0.2,
        0.8, 0.3, 0.2,
        0.8, 0.3, 0.2,
        0.8, 0.3, 0.2,
        0.8, 0.3, 0.2,
        0.8, 0.3, 0.2,

        0.3, 0.7, 0.2,
        0.3, 0.7, 0.2,
        0.3, 0.7, 0.2,
        0.3, 0.7, 0.2,
        0.3, 0.7, 0.2,
        0.3, 0.7, 0.2,

        0.3, 0.5, 0.4,
        0.3, 0.5, 0.4,
        0.3, 0.5, 0.4,
        0.3, 0.5, 0.4,
        0.3, 0.5, 0.4,
        0.3, 0.5, 0.4,

        0.8, 0.5, 0.2,
        0.8, 0.5, 0.2,
        0.8, 0.5, 0.2,
        0.8, 0.5, 0.2,
        0.8, 0.5, 0.2,
        0.8, 0.5, 0.2,
    ];

    let indices = vec![
        0, 1, 2,
        3, 4, 5,
        6, 7, 8,
        9, 10, 11,
        12, 13, 14,
        15, 16, 17,
        18, 19, 20,
        21, 22, 23,
        24, 25, 26,
        27, 28, 29,
        30, 31, 32,
        33, 34, 35
    ];

    (points, colors, indices)
}
