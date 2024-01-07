use std::io;
use tobj;

#[derive(Debug, thiserror::Error)]
pub enum ObjectError {
    #[error("there was a problem reading from file")]
    ProblemReadingFile(
        #[from]
        #[source]
        io::Error
    ),

    #[error("there was a problem loading wavefront file")]
    ProblemLoadingWavefrontObj(
        #[from]
        #[source]
        tobj::LoadError
    )
}

pub fn shader_path() -> String {
    "assets/shaders/".to_string()
}

pub fn texture_path() -> String {
    "assets/textures/".to_string()
}

pub fn object_path() -> String {
    "assets/objects/".to_string()
}

pub fn load_obj_file(
    file_name: &str
) -> Result<(Vec<tobj::Model>, Vec<tobj::Material>), ObjectError> {
    let (models, materials) = tobj::load_obj(
        format!("{}{file_name}.obj", object_path()),
        &tobj::GPU_LOAD_OPTIONS
    )?;

    let materials = materials?;

    Ok((models, materials))
}
