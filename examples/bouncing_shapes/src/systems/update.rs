use engine::{
    Registry,
    components::{
        CVelocity,
        CTransform,
        CDimensions
    }, systems::mvp_matrices::s_set_model_matrix,
};

use crate::{
    HEIGHT,
    WIDTH
};

pub fn s_update(
    registry: &mut Registry,
    delta: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let quads = registry.get_entities_by_tag("quad");

    for quad in quads {
        let Some(vel)       = registry.get_component::<CVelocity>(&quad)    else { continue };
        let Some(transform) = registry.get_component::<CTransform>(&quad)   else { continue };
        let Some(translate) = transform.translate                           else { continue };
        let Some(dims)      = registry.get_component::<CDimensions>(&quad)  else { continue };
        
        let scale = transform.scale.unwrap();

        let vel = glm::vec3(vel.x, vel.y, 0.0);
        let translate = translate + (vel * delta);
        let (colided_x, colided_y) = check_screen_collision(
            translate,
            dims.width * scale.x,
            dims.height * scale.y
        );

        let Some(transform) = registry.get_component_mut::<CTransform>(&quad) else { continue };
        transform.translate = Some(translate);

        let Some(vel) = registry.get_component_mut::<CVelocity>(&quad) else { continue };
        if colided_x { vel.x *= -1.0 }
        if colided_y { vel.y *= -1.0 }

        s_set_model_matrix(&quad, registry);
    }

    Ok(())
}

fn check_screen_collision(
    pos: glm::Vec3,
    w: f32,
    h: f32
) -> (bool, bool) {
    let offset_x = w / 2.0;
    let offset_y = h / 2.0;

    let colided_x = pos.x <= (0.0 + offset_x) || pos.x >= (WIDTH as f32 - offset_x);
    let colided_y = pos.y >= (HEIGHT as f32 - offset_y) || pos.y <= (0.0 + offset_y);

    // println!("{:?}", pos);

    (colided_x, colided_y)
}
