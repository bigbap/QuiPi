use quipi::{
    Registry,
    schema::rect::DEFAULT_RECT_TAG,
    components::{
        CTransform,
        CModelMatrix
    }
};

pub fn update_frame(
    registry: &mut Registry,
) {
    for entity in registry.get_entities_by_tag(DEFAULT_RECT_TAG) {
        let Some(transform) = registry.get_component::<CTransform>(&entity) else { continue; };

        let model = transform.to_matrix();

        if let Some(model_matrix) = registry.get_component_mut::<CModelMatrix>(&entity) {
            model_matrix.0 = model;
        }
    }

}
