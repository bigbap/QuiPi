use std::io;

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

pub fn load_obj_file(
    full_path: &str
) -> Result<(Vec<tobj::Model>, Vec<tobj::Material>), ObjectError> {
    let (models, materials) = tobj::load_obj(
        full_path,
        &tobj::GPU_LOAD_OPTIONS
    )?;

    let materials = materials?;

    Ok((models, materials))
}