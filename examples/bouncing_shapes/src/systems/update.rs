use quipi::{
    components::{
        CBoundingBox2D,
        CTag,
        CTransform2D,
        CVelocity2D
    },
    FrameState, Registry
};
use quipi_core::core::canvas::get_dimensions;

pub fn update(
    registry: &mut Registry,
    frame_state: &mut FrameState
) -> Result<(), Box<dyn std::error::Error>> {
    if frame_state.editor_mode {
        return Ok(())
    }

    let quads = registry.entities.query::<CTag>(CTag { tag: "bubble".into() });

    for quad in quads {
        let Some(vel)       = registry.entities.get::<CVelocity2D>(&quad)    else { continue };
        let Some(transform) = registry.entities.get::<CTransform2D>(&quad)   else { continue };
        let Some(b_box)     = registry.entities.get::<CBoundingBox2D>(&quad) else { continue };
        
        let scale = transform.scale;

        let vel = glm::vec2(vel.x, vel.y);
        let translate = transform.translate + (vel * frame_state.delta);
        let w = b_box.right * scale.x;
        let h = b_box.bottom * scale.y;
        let (colided_x, colided_y) = check_screen_collision(
            translate,
            w,
            h
        );

        let (_x, _y, width, height) = get_dimensions();
        let Some(transform) = registry.entities.get_mut::<CTransform2D>(&quad) else { continue };
        match colided_x {
            -1 => transform.translate.x = 0.0 + w * 0.5,
            1 => transform.translate.x = width as f32 - w * 0.5,
            _ => transform.translate.x = translate.x
        }
        match colided_y {
            -1 => transform.translate.y = 0.0 + h * 0.5,
            1 => transform.translate.y = height as f32 - h * 0.5,
            _ => transform.translate.y = translate.y
        }

        let Some(vel) = registry.entities.get_mut::<CVelocity2D>(&quad) else { continue };
        if colided_x != 0 { vel.x *= -1.0 }
        if colided_y != 0 { vel.y *= -1.0 }
    }

    Ok(())
}

fn check_screen_collision(
    pos: glm::Vec2,
    w: f32,
    h: f32
) -> (i32, i32) {
    let offset_x = w * 0.5;
    let offset_y = h * 0.5;

    let (_x, _y, width, height) = get_dimensions();

    let mut colided_x = 0;
    let mut colided_y = 0;

    if pos.x <= (0.0 + offset_x) { colided_x = -1; }
    if pos.x >= (width as f32 - offset_x) { colided_x = 1; }
    if pos.y >= (height as f32 - offset_y) { colided_y = 1; }
    if pos.y <= (0.0 + offset_y) {  colided_y = -1; }

    (colided_x, colided_y)
}
