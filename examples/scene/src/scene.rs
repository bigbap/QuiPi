use engine::{
    gfx::{
        texture,
        object_loader::{
            load_obj_file,
            ObjectConfig
        },
        ElementArrayMesh
    },
    VersionedIndex,
    Registry,
    components::{
        material::MaterialPart,
        CDirection,
        CPosition,
        CAttenuation,
        CCutoff,
    },
    entity_builders::camera::build_camera_3d,
    systems::material
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
    _shader_id: VersionedIndex,
    _camera_id: VersionedIndex,
    material: CMaterial
) -> Result<Vec<engine::VersionedIndex>, Box<dyn std::error::Error>> {
    // load the object data
    let asset_path = config::asset_path()?.into_os_string().into_string().unwrap();
    let (models_obj, _materials_obj) = load_obj_file(format!("{}/objects/crate.obj", asset_path))?;
    let model_configs = ObjectConfig::from_obj(models_obj)?;


    let transforms = [
        (glm::vec3(-1.0, 0.0, 0.0), 0.0),
        (glm::vec3(0.1, 0.0, 0.1), 0.1),
        (glm::vec3(-0.3, 1.0, 0.2), 0.02),

        (glm::vec3(-3.0, 0.0, 2.0), 0.0),
        (glm::vec3(-1.9, 0.0, 2.1), 0.1),
        (glm::vec3(-2.3, 1.0, 2.2), 0.02),

        (glm::vec3(1.0, 0.0, -2.0), 0.0),
        (glm::vec3(2.1, 0.0, -2.1), 0.1),
        (glm::vec3(1.7, 1.0, -2.2), 0.02),
    ];

    let mut entities = vec![];
    for config in model_configs.iter() {
        for transform in transforms.iter() {
            let mesh = ElementArrayMesh::new(&config.indices)?;
            mesh
                .create_vbo_at(&config.points, 0, 3)?
                .create_vbo_at(&config.texture_coords, 2, 2)?;

            entities.push(registry.create_entity()?
                .with(CMesh { mesh })?
                .with(CTransform {
                    translate: Some(transform.0),
                    scale: Some(glm::vec3(0.5, 0.5, 0.5)),
                    rotate: Some(glm::vec3(0.0, 2.0, 0.0)),
                    angle: transform.1
                })?
                .with(material)?
                .done()?
            )
        }
    }

    Ok(entities)
}

pub fn create_camera(
    registry: &mut engine::Registry,
    width: f32,
    height: f32
) -> Result<engine::VersionedIndex, Box<dyn std::error::Error>> {
    build_camera_3d(
        registry,
        (0.0, 1.0, 5.0),
        90.0,
        width / height,
        0.1,
        100.0
    )
}

pub fn create_texture(
    registry: &mut Registry,
    image_file: &str,
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    registry.create_resource(Texture {
        id: texture::from_image(image_file)?,
    })
}

pub fn directional_light(
    registry: &mut Registry,
    obj_shader_id: VersionedIndex,
    model_config: &ObjectConfig
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    let shader = registry.get_resource::<Shader>(&obj_shader_id)
        .unwrap()
        .program();

    let mat = CMaterial {
        ambient: MaterialPart::Value(0.05, 0.05, 0.05),
        diffuse: MaterialPart::Value(0.1, 0.1, 0.1),
        specular: MaterialPart::Value(0.5, 0.5, 0.5),
        shininess: 0.0
    };
    
    let direction = (-0.8, -0.1, -0.1);

    if let Some(ambient) = material::s_get_value(&mat.ambient) {
        shader.set_float_3("dirLight.ambient", ambient);
    }
    if let Some(diffuse) = material::s_get_value(&mat.diffuse) {
        shader.set_float_3("dirLight.diffuse", diffuse);
    }
    if let Some(specular) = material::s_get_value(&mat.specular) {
        shader.set_float_3("dirLight.ambient", specular);
    }
    shader.set_float_3("dirLight.direction", direction);

    let mesh = ElementArrayMesh::new(&model_config.indices)?;
    mesh.create_vbo_at(&model_config.points, 0, 3)?;

    registry.create_entity()?
        .with(CDirection {
            x: direction.0,
            y: direction.1,
            z: direction.2
        })?
        .with(CRGBA { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })?
        .with(CMesh { mesh })?
        .with(CTransform {
            translate: Some(glm::vec3(7.0, 10.0, 0.0)),
            ..CTransform::default()
        })?
        .with(mat)?
        .done()
}

pub fn point_light(
    registry: &mut Registry,
    obj_shader_id: VersionedIndex,
    model_config: &ObjectConfig
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    let shader = registry.get_resource::<Shader>(&obj_shader_id)
        .unwrap()
        .program();

    let mat = CMaterial {
        ambient: MaterialPart::Value(1.0, 0.0, 0.0),
        diffuse: MaterialPart::Value(1.0, 0.0, 0.0),
        specular: MaterialPart::Value(1.0, 0.2, 0.2),
        shininess: 0.0
    };

    let position = CPosition {
        x: 5.0,
        y: 1.0,
        z: 6.0
    };
    let attenuation = CAttenuation {
        constant: 1.0,
        linear: 0.09,
        quadratic: 0.032,
    };

    if let Some(ambient) = material::s_get_value(&mat.ambient) {
        shader.set_float_3("pointLight.ambient", ambient);
    }
    if let Some(diffuse) = material::s_get_value(&mat.diffuse) {
        shader.set_float_3("pointLight.diffuse", diffuse);
    }
    if let Some(specular) = material::s_get_value(&mat.specular) {
        shader.set_float_3("pointLight.ambient", specular);
    }
    shader.set_float_3("pointLight.position", (position.x, position.y, position.z));
    shader.set_float("pointLight.constant", attenuation.constant);
    shader.set_float("pointLight.linear", attenuation.linear);
    shader.set_float("pointLight.quadratic", attenuation.quadratic);

    let mesh = ElementArrayMesh::new(&model_config.indices)?;
    mesh.create_vbo_at(&model_config.points, 0, 3)?;

    registry.create_entity()?
        .with(position)?
        .with(attenuation)?
        .with(mat)?
        .with(CRGBA { r: 0.6, g: 0.0, b: 0.0, a: 1.0 })?
        .with(CMesh { mesh })?
        .with(CTransform {
            translate: Some(glm::vec3(5.0, 1.0, 6.0)),
            scale: Some(glm::vec3(0.2, 0.2, 0.2)),
            ..CTransform::default()
        })?
        .done()
}

pub fn spot_light(
    registry: &mut Registry,
    obj_shader_id: VersionedIndex,
    model_config: &ObjectConfig
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    let shader = registry.get_resource::<Shader>(&obj_shader_id)
        .unwrap()
        .program();

    let mat = CMaterial {
        ambient: MaterialPart::Value(0.1, 0.1, 0.1),
        diffuse: MaterialPart::Value(0.5, 0.5, 0.5),
        specular: MaterialPart::Value(1.0, 1.0, 1.0),
        shininess: 0.0
    };

    let attenuation = CAttenuation {
        constant: 1.0,
        linear: 0.09,
        quadratic: 0.032,
    };

    let cutoffs = CCutoff {
        inner_cutoff: 12.5_f32.to_radians().cos(),
        outer_cutoff: 17.5_f32.to_radians().cos()
    };

    if let Some(ambient) = material::s_get_value(&mat.ambient) {
        shader.set_float_3("spotLight.ambient", ambient);
    }
    if let Some(diffuse) = material::s_get_value(&mat.diffuse) {
        shader.set_float_3("spotLight.diffuse", diffuse);
    }
    if let Some(specular) = material::s_get_value(&mat.specular) {
        shader.set_float_3("spotLight.ambient", specular);
    }
    shader.set_float("spotLight.constant", attenuation.constant);
    shader.set_float("spotLight.linear", attenuation.linear);
    shader.set_float("spotLight.quadratic", attenuation.quadratic);
    shader.set_float("spotLight.cutOff", cutoffs.inner_cutoff);
    shader.set_float("spotLight.outerCutOff", cutoffs.outer_cutoff);

    let mesh = ElementArrayMesh::new(&model_config.indices)?;
    mesh.create_vbo_at(&model_config.points, 0, 3)?;

    registry.create_entity()?
        .with(CRGBA { r: 0.6, g: 0.0, b: 0.0, a: 1.0 })?
        .with(CPosition { x: 0.0, y: 0.0, z: 0.0 })?
        .with(CDirection { x: 0.0, y: 0.0, z: 0.0 })?
        .with(attenuation)?
        .with(cutoffs)?
        .with(mat)?
        .with(CMesh { mesh })?
        .with(CTransform {
            translate: Some(glm::vec3(5.0, 1.0, 6.0)),
            scale: Some(glm::vec3(0.2, 0.2, 0.2)),
            ..CTransform::default()
        })?
        .done()
}
