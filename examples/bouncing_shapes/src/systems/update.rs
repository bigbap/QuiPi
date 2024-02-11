use quipi::{
    components::{
        CBoundingBox2D,
        CTag,
        CTransform2D,
        CVelocity2D
    },
    schemas::entity2d::DEFAULT_RECT_TAG,
    FrameState, Registry
};

use crate::{
    HEIGHT,
    WIDTH
};

pub fn update(
    registry: &mut Registry,
    frame_state: &mut FrameState
) -> Result<(), Box<dyn std::error::Error>> {
    if frame_state.editor_mode {
        return Ok(())
    }

    let quads = registry.entities.query::<CTag>(CTag { tag: DEFAULT_RECT_TAG.to_string() });

    for quad in quads {
        let Some(vel)       = registry.entities.get::<CVelocity2D>(&quad)    else { continue };
        let Some(transform) = registry.entities.get::<CTransform2D>(&quad)   else { continue };
        let Some(b_box)     = registry.entities.get::<CBoundingBox2D>(&quad) else { continue };
        
        let scale = transform.scale;

        let vel = glm::vec2(vel.x, vel.y);
        let translate = transform.translate + (vel * frame_state.delta);
        let (colided_x, colided_y) = check_screen_collision(
            translate,
            b_box.right * scale.x,
            b_box.bottom * scale.y
        );

        let Some(transform) = registry.entities.get_mut::<CTransform2D>(&quad) else { continue };
        transform.translate = translate;

        let Some(vel) = registry.entities.get_mut::<CVelocity2D>(&quad) else { continue };
        if colided_x { vel.x *= -1.0 }
        if colided_y { vel.y *= -1.0 }
    }

    Ok(())
}

fn check_screen_collision(
    pos: glm::Vec2,
    w: f32,
    h: f32
) -> (bool, bool) {
    let offset_x = w * 0.5;
    let offset_y = h * 0.5;

    let colided_x = pos.x <= (0.0 + offset_x) || pos.x >= (WIDTH as f32 - offset_x);
    let colided_y = pos.y >= (HEIGHT as f32 - offset_y) || pos.y <= (0.0 + offset_y);

    (colided_x, colided_y)
}
