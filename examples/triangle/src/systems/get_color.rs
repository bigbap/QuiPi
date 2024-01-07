use engine::{
    VersionedIndex,
    Registry
};
use crate::components::ColorComponent;

pub fn get_color(
    ticks: f32,
    entity: &VersionedIndex,
    registry: &Registry
) -> (f32, f32, f32, f32) {
    match registry.components.borrow_mut().get_component::<ColorComponent>(entity) {
        Some(color) => {
            color.0 = ticks.sin();
            color.1 = ticks.cos();
            color.2 = ticks.sin();

            (
                color.0,
                color.1,
                color.2,
                color.3
            )
        },
        None => (1.0, 0.0, 0.0, 1.0)
    }

}
