use std::{
    fs::{
        self,
        File
    },
    io::BufReader
};

use crate::{
    components::{
        CName,
        CTag,
        CTransform,
        CVelocity,
        CCamera,
        CBoundingBox,
        CRect,
        CRGBA,
        CEulerAngles,
        CGizmo3D,
        CDirection,
    },
    get_components,
    schemas::SchemaScene,
    utils::to_abs_path,
    Registry
};

pub fn save_entities(
    name: &str,
    registry: &mut Registry
) -> Result<(), Box<dyn std::error::Error>> {
    let mut to_save = String::new();
    for entity in registry.entities.get_valid_entities() {
        let list = get_components!(
            registry.entities,
            &entity,
            CName,
            CTag,
            CTransform,
            CVelocity,
            CCamera,
            CBoundingBox,
            CRect,
            CRGBA,
            CEulerAngles,
            CGizmo3D,
            CDirection,
            CDirection,
        );

        to_save.push_str(&serde_yaml::to_string(&list)?);
    }

    let path = to_abs_path(&format!("assets/scenes/{}.yaml", name))?; 

    fs::write(path, to_save)?;

    Ok(())
}

pub fn save_scene(
    name: &str,
    scene: &SchemaScene,
) -> Result<(), Box<dyn std::error::Error>> {
    let str = serde_yaml::to_string(&scene)?;
    let path = to_abs_path(&format!("assets/scenes/{}.yaml", name))?; 

    fs::write(path, str)?;

    Ok(())
}

pub fn load_scene(
    name: &str,
    default: SchemaScene
) -> Result<SchemaScene, Box<dyn std::error::Error>> {
    let path = to_abs_path(&format!("assets/scenes/{}.yaml", name))?; 
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);

        return Ok(serde_yaml::from_reader(reader)?);
    }
    
    Ok(default)
}
