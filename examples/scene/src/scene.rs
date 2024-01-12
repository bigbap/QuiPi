use engine::{
    resources::CameraProjection,
    gfx::{
        Texture,
        object_loader::{
            load_obj_file,
            ObjectConfig
        },
    },
    VersionedIndex,
    Registry
};

use crate::{
    resources::*,
    components::*,
    CONFIG
};

pub fn create_registry() -> Result<engine::Registry, Box<dyn std::error::Error>> {
    let mut registry = engine::Registry::init()?;

    register_resources(&mut registry);
    register_components(&mut registry);

    Ok(registry)
}

pub fn create_crate(
    registry: &mut Registry,
    camera: VersionedIndex
) -> Result<Vec<engine::VersionedIndex>, Box<dyn std::error::Error>> {
    // load the object data
    let (models_obj, _materials_obj) = load_obj_file(format!("{}objects/crate.obj", CONFIG.asset_path))?;
    let model_configs = ObjectConfig::from_obj(models_obj)?;

    let shader_program = Shader::new(&format!("{}shaders/simple", CONFIG.asset_path))?;
    let shader = registry.create_resource(
        shader_program
    )?;
    
    let mut entities = vec![];
    for config in model_configs {
        entities.push(registry.create_entity()?
            .with(DrawComponent {
                shader_id: shader,
                textures: create_textures()?
            })?
            .with(MeshComponent::new(&config)?)?
            .with(CameraComponent { id: camera })?
            .with(TransformComponent {
                translate: None,
                scale: Some(glm::vec3(0.5, 0.5, 0.5)),
                rotate: Some(glm::vec3(0.2, 0.3, 0.0)),
                angle: Some(0.1)
            })?
            .done()?
        )
    }

    Ok(entities)
}

pub fn create_camera(
    registry: &mut engine::Registry
) -> Result<engine::VersionedIndex, Box<dyn std::error::Error>> {
    registry.create_resource(Camera3D {
        projection: CameraProjection::Perspective,
        pos: glm::vec3(0.0, 1.0, 5.0),
        front: glm::vec3(0.0, 0.0, -1.0),
        up: glm::vec3(0.0, 1.0, 0.0),

        fov: 75.0,
        aspect_ratio: 800.0/600.0,
        pitch: 0.0,
        yaw: -90.0,

        near_plane: 0.1,
        far_plane: 100.0
    })
}

fn create_textures() -> Result<Vec<(String, Texture)>, Box<dyn std::error::Error>> {
    Ok(vec![
        ("material.diffuse".to_string(), Texture::from_image(&format!("{}objects/textures/container.png", CONFIG.asset_path))?),
        ("material.specular".to_string(), Texture::from_image(&format!("{}objects/textures/container_specular.png", CONFIG.asset_path))?)
    ])
}
