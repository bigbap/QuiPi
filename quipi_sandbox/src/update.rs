use quipi::{
    components::{
        CModelMatrix,
        CTag,
        CTransform
    }, get_components, schemas::rect::DEFAULT_RECT_TAG, Registry
};

pub fn update_frame(
    registry: &mut Registry,
) {
    for entity in registry.entities.query(CTag { tag: DEFAULT_RECT_TAG.to_string() }) {
        let Some(transform) = registry.entities.get::<CTransform>(&entity) else { continue; };

        let lst = get_components!(&registry.entities, &entity, CTransform,);

        println!("{:?}", lst);

        let model = transform.to_matrix();

        if let Some(model_matrix) = registry.entities.get_mut::<CModelMatrix>(&entity) {
            model_matrix.0 = model;
        }
    }

}
