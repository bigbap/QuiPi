use quipi::{
    GlobalRegistry,
    systems::{
        rendering::{
            texture::*,
            mesh::{
                ElementArrayMesh,
                ShaderLocation
            }
        },
        mvp_matrices::s_set_model_matrix,
        assets::{
            obj_loader,
            ObjectConfig
        }
    },
    resources::Texture,
    wrappers::opengl::buffer::BufferUsage,
    components::{
        CMaterial,
        material::MaterialPart,
        CModelNode,
        CTransform,
        CModelMatrix
    },
    utils::to_abs_path
};

pub fn s_load_scene(
    registry: &mut GlobalRegistry
) -> Result<(), Box<dyn std::error::Error>> {
    let (models, _) = obj_loader::s_load_obj_file(to_abs_path("assets/objects/cube.obj")?)?;
    let obj_configs = ObjectConfig::from_obj(models)?;
    let texture = Texture(from_image(&to_abs_path("assets/objects/textures/tex.png")?)?);

    let texture = registry.create_resource(texture)?;

    for x in 0..50 {
        for y in 0..50 {
            for config in &obj_configs {
                let mut mesh = ElementArrayMesh::new(
                    config.indices.len(),
                    BufferUsage::StaticDraw,
                )?;
                mesh
                    .with_ebo(&config.indices)?
                    .with_vbo::<3, f32>(ShaderLocation::Zero, &config.points)?
                    .with_vbo::<2, f32>(ShaderLocation::One, &config.texture_coords)?;

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
                        translate: glm::vec3(
                            0.0 + (1.0 * x as f32),
                            0.0,
                            0.0 + (1.0 * y as f32),
                        ),
                        scale: Some(glm::vec3(0.5, 0.5, 0.5)),
                        ..CTransform::default()
                    })?
                    .with(CModelMatrix::default())?
                    .done()?;

                s_set_model_matrix(&cube, registry);
            }
        }
    }

    Ok(())
}
