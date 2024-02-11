use quipi::{
    components::{
        CModelMatrix2D,
        CTag,
        CTransform2D
    },
    schemas::entity2d::DEFAULT_RECT_TAG,
    Registry
};

pub fn update_frame(
    registry: &mut Registry,
) {
    for entity in registry.entities.query(CTag { tag: DEFAULT_RECT_TAG.to_string() }) {
        let Some(transform) = registry.entities.get::<CTransform2D>(&entity) else { continue; };

        let model = transform.to_matrix();

        if let Some(model_matrix) = registry.entities.get_mut::<CModelMatrix2D>(&entity) {
            model_matrix.0 = model;
        }
    }
}
