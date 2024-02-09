use std::{
    fs::{
        self,
        File
    },
    io::BufReader
};

use crate::{
    schemas::SchemaScene,
    utils::to_abs_path,
};

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
