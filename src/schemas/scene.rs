use std::{
    fs::{
        self,
        File
    },
    io::BufReader
};
use crate::QPResult;
use crate::prelude::{
    QPError,
    qp_core::to_abs_path,
    qp_schemas::SchemaScene2D,
    qp_data::ISchema,
    GlobalRegistry,
    VersionedIndex,
};

pub fn save_scene_2d(
    name: &str,
    scene: VersionedIndex,
    registry: &GlobalRegistry
) -> QPResult<()> {
    if let Some(scene) = SchemaScene2D::from_entity(scene, registry) {
        let str = serde_yaml::to_string(&scene)
            .map_err(|e| QPError::Generic(e.to_string()))?;
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
) -> QPResult<SchemaScene2D> {
    let path = to_abs_path(&format!("assets/scenes/{}.yaml", name))?;
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);

        return Ok(
            serde_yaml::from_reader(reader)
                .map_err(|e| QPError::Generic(e.to_string()))?
        );
    }
    
    Ok(default)
}
