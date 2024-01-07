use engine::{
    VersionedIndex,
    ComponentRegistry
};
use crate::components::ColorComponent;

pub fn get_color(
    ticks: f32,
    entity: &VersionedIndex,
    ecs: &mut ComponentRegistry
) -> (f32, f32, f32, f32) {
    let color = ecs.get_component::<ColorComponent>(entity).unwrap();

    color.0 = ticks.sin();
    color.1 = ticks.cos();
    color.2 = ticks.sin();

    (
        color.0,
        color.1,
        color.2,
        color.3
    )
}
