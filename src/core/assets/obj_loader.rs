use std::io;

use crate::core::utils::to_abs_path;

#[derive(Debug, thiserror::Error)]
pub enum ObjectError {
    #[error("there was a problem reading from file")]
    ProblemReadingFile(
        #[from]
        #[source]
        io::Error
    ),

    #[error("there was a problem loading wavefront file")]
    ProblemLoadingWavefrontObj {
        #[from]
        #[source]
        from: tobj::LoadError,
    },

    #[error("there was a problem loading wavefront file")]
    ProblemLoadingGltf {
        #[from]
        #[source]
        from: gltf::Error,
    }
}

pub fn load_obj_file(
    rel_path: String
) -> Result<(Vec<tobj::Model>, Vec<tobj::Material>), ObjectError> {
    let full_path = to_abs_path(&rel_path)?;
    let (models, materials) = tobj::load_obj(
        full_path,
        &tobj::GPU_LOAD_OPTIONS
    )?;

    let materials = materials?;

    Ok((models, materials))
}
