use std::{
    fs::{
        self,
        File
    },
    io::BufReader
};

use crate::prelude::{
    qp_core::to_abs_path,
    qp_schemas::SchemaScene2D,
    qp_data::ISchema,
    Registry,
    VersionedIndex,
};

pub fn save_scene_2d(
    name: &str,
    scene: VersionedIndex,
    registry: &Registry
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(scene) = SchemaScene2D::from_entity(scene, registry) {
        let str = serde_yaml::to_string(&scene)?;
        let path = to_abs_path(&format!("assets/scenes/{}.yaml", name))?; 

        fs::write(path, str)?;
    } else {
        #[cfg(debug_assertions)]
        println!("there was a problem saving the scene");
    }

    Ok(())
}

pub fn load_scene_2d(
    name: &str,
    default: SchemaScene2D
) -> Result<SchemaScene2D, Box<dyn std::error::Error>> {
    let path = to_abs_path(&format!("assets/scenes/{}.yaml", name))?;
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);

        return Ok(serde_yaml::from_reader(reader)?);
    }
    
    Ok(default)
}
