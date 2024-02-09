use quipi::{
    components::CTag, schemas::{
        camera::DEFAULT_CAMERA_TAG, entity::DEFAULT_RECT_TAG
    }, systems::rendering::draw::s_draw_by_tag, wrappers::opengl::{
        capabilities::{
            gl_blending_func, gl_enable, GLBlendingFactor, GLCapability
        },
        draw::DrawMode
    }, Registry
};

pub fn draw_frame(
    registry: &mut Registry,
) -> Result<(), Box<dyn std::error::Error>> {
    gl_enable(GLCapability::AlphaBlending);
    gl_blending_func(GLBlendingFactor::SrcAlpha, GLBlendingFactor::OneMinusSrcAlpha);

    let cameras = registry.entities.query::<CTag>(CTag { tag: DEFAULT_CAMERA_TAG.to_string() });
    for camera in cameras {
        s_draw_by_tag(
            DEFAULT_RECT_TAG,
            registry,
            &camera,
            DrawMode::Triangles
        )?;
    }

    Ok(())
}
