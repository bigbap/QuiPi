use engine::{
    Registry,
    components::{
        CVelocity,
        CPosition
    }
};

pub fn s_update(
    registry: &mut Registry,
    delta: f32
) {
    let quads = registry.get_entities_by_tag("quad");

    for entity in quads {
        let vel = registry.get_component::<CVelocity>(&entity).unwrap();
        let vel = (vel.x, vel.y);

        let pos = registry.get_component_mut::<CPosition>(&entity).unwrap();

        pos.x += vel.0 * delta;
        pos.y += vel.1 * delta;
    }
}
