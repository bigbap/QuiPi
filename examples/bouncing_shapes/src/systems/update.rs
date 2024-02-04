use quipi::{
    Registry,
    components::{
        CVelocity,
        CTransform,
        CBoundingBox, CModelMatrix,
    },
    AppState,
    schema::rect::DEFAULT_RECT_TAG,
};

use crate::{
    HEIGHT,
    WIDTH
};

pub fn s_update(
    frame_state: &AppState,
    registry: &mut Registry,
) -> Result<(), Box<dyn std::error::Error>> {
    if frame_state.editor_mode {
        return Ok(())
    }

    let quads = registry.get_entities_by_tag(DEFAULT_RECT_TAG);

    for quad in quads {
        let Some(vel)       = registry.get_component::<CVelocity>(&quad)    else { continue };
        let Some(transform) = registry.get_component::<CTransform>(&quad)   else { continue };
        let Some(b_box)     = registry.get_component::<CBoundingBox>(&quad) else { continue };
        
        let scale = transform.scale.unwrap_or(glm::vec3(1.0, 1.0, 1.0));

        let vel = glm::vec3(vel.x, vel.y, 0.0);
        let translate = transform.translate + (vel * frame_state.delta);
        let (colided_x, colided_y) = check_screen_collision(
            translate,
            b_box.right * scale.x,
            b_box.bottom * scale.y
        );

        let Some(transform) = registry.get_component_mut::<CTransform>(&quad) else { continue };
        transform.translate = translate;
        let matrix = transform.to_matrix();

        let Some(vel) = registry.get_component_mut::<CVelocity>(&quad) else { continue };
        if colided_x { vel.x *= -1.0 }
        if colided_y { vel.y *= -1.0 }

        let Some(model) = registry.get_component_mut::<CModelMatrix>(&quad) else { continue };
        model.update_model_matrix(matrix);
    }

    Ok(())
}

fn check_screen_collision(
    pos: glm::Vec3,
    w: f32,
    h: f32
) -> (bool, bool) {
    let offset_x = w * 0.5;
    let offset_y = h * 0.5;

    let colided_x = pos.x <= (0.0 + offset_x) || pos.x >= (WIDTH as f32 - offset_x);
    let colided_y = pos.y >= (HEIGHT as f32 - offset_y) || pos.y <= (0.0 + offset_y);

    (colided_x, colided_y)
}
