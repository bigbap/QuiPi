use engine::{
    resources::{
        CameraProjection,
        texture::TextureType,
    },
    gfx::{
        texture,
        object_loader::{
            load_obj_file,
            ObjectConfig
        },
        Material,
        material::MaterialPart
    },
    VersionedIndex,
    Registry,
    components::{
        transform::Transforms,
        LightDirectionalComponent, LightPointComponent, LightSpotComponent,
    }
};

use crate::{
    resources::*,
    components::*,
    config,
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
    camera_id: VersionedIndex,
    material: Material
) -> Result<Vec<engine::VersionedIndex>, Box<dyn std::error::Error>> {
    // load the object data
    let asset_path = config::asset_path()?.into_os_string().into_string().unwrap();
    let (models_obj, _materials_obj) = load_obj_file(format!("{}/objects/crate.obj", asset_path))?;
    let model_configs = ObjectConfig::from_obj(models_obj)?;

    let shader = registry.get_resource::<Shader>(&shader_id).unwrap();
    shader.program().use_program();
    if let Some(diffuse) = material.get_texture(&material.diffuse, registry) {
        shader.program().set_int("material.diffuse", diffuse.index);
    }
    if let Some(specular) = material.get_texture(&material.specular, registry) {
        shader.program().set_int("material.specular", specular.index);
    }

    shader.program().set_float("material.shininess", 0.6 * 128.0);
    
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
                camera_id,
                materials: vec![material],
                ..DrawComponent::default()
            })?
            .with(MeshComponent::new(config)?)?
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
        position: glm::vec3(0.0, 1.0, 5.0),
        
        fov: 90.0,
        aspect_ratio: 800.0 / 600.0,
        ..Camera3D::default()
    })
}

pub fn create_texture(
    registry: &mut Registry,
    image_file: &str,
    kind: TextureType,
    index: i32,
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    registry.create_resource(Texture {
        id: texture::from_image(image_file)?,
        kind,
        index
    })
}

pub fn directional_light(
    registry: &mut Registry,
    light_shader_id: VersionedIndex,
    obj_shader_id: VersionedIndex,
    camera_id: VersionedIndex,
    model_config: &ObjectConfig
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    let shader = registry.get_resource::<Shader>(&obj_shader_id)
        .unwrap()
        .program();

    let material = Material {
        ambient: MaterialPart::Color(0.05, 0.05, 0.05),
        diffuse: MaterialPart::Color(0.1, 0.1, 0.1),
        specular: MaterialPart::Color(0.5, 0.5, 0.5),
        shininess: 0.0
    };
    let light = LightDirectionalComponent {
        direction: (-0.8, -0.1, -0.1),
        material,
    };

    if let Some(ambient) = material.get_color(&material.ambient) {
        shader.set_float_3("dirLight.ambient", ambient);
    }
    if let Some(diffuse) = material.get_color(&material.diffuse) {
        shader.set_float_3("dirLight.diffuse", diffuse);
    }
    if let Some(specular) = material.get_color(&material.specular) {
        shader.set_float_3("dirLight.ambient", specular);
    }
    shader.set_float_3("dirLight.direction", light.direction);

    registry.create_entity()?
        .with(light)?
        .with(DrawComponent {
            shader_id: light_shader_id,
            camera_id,
            materials: vec![],
            color: Some((1.0, 1.0, 1.0))
        })?
        .with(MeshComponent::new(model_config)?)?
        .with(TransformComponent {
            transforms: vec![
                Transforms {
                    translate: Some(glm::vec3(7.0, 10.0, 0.0)),
                    ..Transforms::default()
                }
            ]
        })?
        .done()
}

pub fn point_light(
    registry: &mut Registry,
    light_shader_id: VersionedIndex,
    obj_shader_id: VersionedIndex,
    camera_id: VersionedIndex,
    model_config: &ObjectConfig
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    let shader = registry.get_resource::<Shader>(&obj_shader_id)
        .unwrap()
        .program();

    let material = Material {
        ambient: MaterialPart::Color(1.0, 0.0, 0.0),
        diffuse: MaterialPart::Color(1.0, 0.0, 0.0),
        specular: MaterialPart::Color(1.0, 0.2, 0.2),
        shininess: 0.0
    };
    let light = LightPointComponent {
        position: (5.0, 1.0, 6.0),
        material,
        constant: 1.0,
        linear: 0.09,
        quadratic: 0.032,
    };

    if let Some(ambient) = material.get_color(&material.ambient) {
        shader.set_float_3("pointLight.ambient", ambient);
    }
    if let Some(diffuse) = material.get_color(&material.diffuse) {
        shader.set_float_3("pointLight.diffuse", diffuse);
    }
    if let Some(specular) = material.get_color(&material.specular) {
        shader.set_float_3("pointLight.ambient", specular);
    }
    shader.set_float_3("pointLight.position", light.position);
    shader.set_float("pointLight.constant", light.constant);
    shader.set_float("pointLight.linear", light.linear);
    shader.set_float("pointLight.quadratic", light.quadratic);

    registry.create_entity()?
        .with(light)?
        .with(DrawComponent {
            shader_id: light_shader_id,
            camera_id,
            materials: vec![],
            color: Some((0.6, 0.0, 0.0))
        })?
        .with(MeshComponent::new(model_config)?)?
        .with(TransformComponent {
            transforms: vec![
                Transforms {
                    translate: Some(glm::vec3(5.0, 1.0, 6.0)),
                    scale: Some(glm::vec3(0.2, 0.2, 0.2)),
                    ..Transforms::default()
                }
            ]
        })?
        .done()
}

pub fn spot_light(
    registry: &mut Registry,
    light_shader_id: VersionedIndex,
    obj_shader_id: VersionedIndex,
    camera_id: VersionedIndex,
    model_config: &ObjectConfig
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    let shader = registry.get_resource::<Shader>(&obj_shader_id)
        .unwrap()
        .program();

    let material = Material {
        ambient: MaterialPart::Color(0.1, 0.1, 0.1),
        diffuse: MaterialPart::Color(0.5, 0.5, 0.5),
        specular: MaterialPart::Color(1.0, 1.0, 1.0),
        shininess: 0.0
    };
    let light = LightSpotComponent {
        position: (0.0, 0.0, 0.0),
        direction: (0.0, 0.0, 0.0),
        material,
        constant: 1.0,
        linear: 0.09,
        quadratic: 0.032,
        inner_cutoff: 12.5_f32.to_radians().cos(),
        outer_cutoff: 17.5_f32.to_radians().cos()
    };

    if let Some(ambient) = material.get_color(&material.ambient) {
        shader.set_float_3("spotLight.ambient", ambient);
    }
    if let Some(diffuse) = material.get_color(&material.diffuse) {
        shader.set_float_3("spotLight.diffuse", diffuse);
    }
    if let Some(specular) = material.get_color(&material.specular) {
        shader.set_float_3("spotLight.ambient", specular);
    }
    shader.set_float("spotLight.constant", light.constant);
    shader.set_float("spotLight.linear", light.linear);
    shader.set_float("spotLight.quadratic", light.quadratic);
    shader.set_float("spotLight.cutOff", light.inner_cutoff);
    shader.set_float("spotLight.outerCutOff", light.outer_cutoff);

    registry.create_entity()?
        .with(light)?
        .with(DrawComponent {
            shader_id: light_shader_id,
            camera_id,
            materials: vec![],
            color: Some((0.6, 0.0, 0.0))
        })?
        .with(MeshComponent::new(model_config)?)?
        .with(TransformComponent {
            transforms: vec![
                Transforms {
                    translate: Some(glm::vec3(5.0, 1.0, 6.0)),
                    scale: Some(glm::vec3(0.2, 0.2, 0.2)),
                    ..Transforms::default()
                }
            ]
        })?
        .done()
}

fn create_transform(translate: glm::Vec3, angle: f32) -> Transforms {
    Transforms {
        translate: Some(translate),
        scale: Some(glm::vec3(0.5, 0.5, 0.5)),
        rotate: Some(glm::vec3(0.0, 2.0, 0.0)),
        angle
    }
}
