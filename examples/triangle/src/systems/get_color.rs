use engine::{
    VersionedIndex,
    Registry,
    components::CRGBA
};

pub fn get_color(
    ticks: f32,
    entity: &VersionedIndex,
    registry: &mut Registry
) -> (f32, f32, f32, f32) {
    match registry.get_component_mut::<CRGBA>(entity) {
        Some(color) => {
            color.r = ticks.sin();
            color.g = ticks.cos();
            color.b = ticks.sin();

            (
                color.r,
                color.g,
                color.b,
                color.a
            )
        },
        None => (1.0, 0.0, 0.0, 1.0)
    }

}
