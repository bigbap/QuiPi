use crate::{
    components::TransformComponent,
    VersionedIndex,
    Registry
};

pub fn apply_transforms(
    entity: &VersionedIndex,
    registry: &Registry
) -> Option<glm::Mat4> {
    let transforms = registry.get_component::<TransformComponent>(entity)?;

    let model = glm::Mat4::identity();
    let model = match transforms.translate {
        Some(translate) => glm::translate(&model, &translate),
        None => model
    };
    let model = match transforms.rotate {
        None => model,
        Some(rotate) => match transforms.angle {
            Some(angle) => glm::rotate(&model, angle, &glm::normalize(&rotate)),
            None => model
        }
    };
    Some(match transforms.scale {
        Some(scale) => glm::scale(&model, &scale),
        None => model
    })
}
