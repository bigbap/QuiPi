use engine::{
    Registry,
    components::{
        CVelocity,
        CTransform
    },
    math::random::Random,
};

use super::spawn_quad::s_spawn_quad;

// use crate::{
//     HEIGHT,
//     WIDTH
// };

pub fn s_update(
    registry: &mut Registry,
    delta: f32,
    rand: &mut Random
) -> Result<(), Box<dyn std::error::Error>> {
    let quads = registry.get_entities_by_tag("quad");

    if rand.random() > 0.93 {
        s_spawn_quad(registry, rand)?;
    }

    for quad in quads {
        let Some(vel) = registry.get_component::<CVelocity>(&quad) else { return Ok(()) };
        let vel = glm::vec3(vel.x, vel.y, 0.0);

        let Some(transform) = registry.get_component_mut::<CTransform>(&quad) else { return Ok(()) };
        let Some(translate) = transform.translate else { return Ok(()) };

        let translate = translate + (vel * delta);

        transform.translate = Some(translate);

        if check_screen_collision(translate) {
            registry.delete_entity(quad)?;

            // println!("{}", registry.entity_count());
        }
    }

    Ok(())
}

fn check_screen_collision(
    pos: glm::Vec3
) -> bool {
    let colided = pos.x <= -1.0 || pos.y >= 1.0 || pos.x >= 1.0 || pos.y <= -1.0;

    colided
}
