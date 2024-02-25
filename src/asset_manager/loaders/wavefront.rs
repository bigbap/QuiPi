use crate::{
    QPResult,
    prelude::qp_core::to_abs_path
};

pub fn load_obj_file(
    rel_path: String
) -> QPResult<(Vec<tobj::Model>, Vec<tobj::Material>)> {
    let full_path = to_abs_path(&rel_path)?;
    let (models, materials) = tobj::load_obj(
        full_path,
        &tobj::GPU_LOAD_OPTIONS
    )?;

    let materials = materials?;

    Ok((models, materials))
}

