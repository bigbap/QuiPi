use engine::{
    resources::{
        CameraProjection,
        texture::TextureType
    },
    gfx::{
        texture,
        object_loader::{
            load_obj_file,
            ObjectConfig
        },
    },
    VersionedIndex,
    Registry,
    components::transform::Transforms
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

pub fn create_crates(
    registry: &mut Registry,
    shader_id: VersionedIndex,
    camera: VersionedIndex,
    textures: Vec<VersionedIndex>
) -> Result<Vec<engine::VersionedIndex>, Box<dyn std::error::Error>> {
    // load the object data
    let (models_obj, _materials_obj) = load_obj_file(format!("{}objects/crate.obj", CONFIG.asset_path))?;
    let model_configs = ObjectConfig::from_obj(models_obj)?;

    let shader = registry.get_resource::<Shader>(&shader_id).unwrap();
    shader.program().use_program();
    for (i, texture_i) in textures.iter().enumerate() {
        let texture = registry.get_resource::<Texture>(texture_i).unwrap();
        
        match texture.kind {
            TextureType::Diffuse => shader.program().set_int("material.diffuse", i as i32),
            TextureType::Specular => shader.program().set_int("material.specular", i as i32)
        }
    }
    
    let transforms = TransformComponent {
        transforms: vec![
            create_transform(glm::vec3(-1.0, 0.0, 0.0), 0.0),
            create_transform(glm::vec3(0.1, 0.0, 0.1), 0.1),
            create_transform(glm::vec3(-0.3, 1.0, 0.2), 0.02),
            
            create_transform(glm::vec3(-3.0, 0.0, 2.0), 0.0),
            create_transform(glm::vec3(-1.9, 0.0, 2.1), 0.1),
            create_transform(glm::vec3(-2.3, 1.0, 2.2), 0.02),
            
            create_transform(glm::vec3(1.0, 0.0, -2.0), 0.0),
            create_transform(glm::vec3(2.1, 0.0, -2.1), 0.1),
            create_transform(glm::vec3(1.7, 1.0, -2.2), 0.02),
        ]
    };

    let mut entities = vec![];
    for config in model_configs.iter() {
        entities.push(registry.create_entity()?
            .with(DrawComponent {
                shader_id,
                textures: textures.to_vec()
            })?
            .with(MeshComponent::new(config)?)?
            .with(CameraComponent { id: camera })?
            .with(transforms.clone())?
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

pub fn create_texture(
    registry: &mut Registry,
    image_file: &str,
    kind: TextureType
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    registry.create_resource(Texture {
        index: texture::from_image(image_file)?,
        kind
    })
}

fn create_transform(translate: glm::Vec3, angle: f32) -> Transforms {
    Transforms {
        translate: Some(translate),
        scale: Some(glm::vec3(0.5, 0.5, 0.5)),
        rotate: Some(glm::vec3(0.0, 2.0, 0.0)),
        angle
    }
}
