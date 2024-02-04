use quipi::{
    Registry,
    wrappers::opengl::{
        capabilities::{
            GLCapability,
            gl_enable,
            gl_blending_func,
            GLBlendingFactor
        },
        draw::DrawMode
    },
    schema::{
        rect::DEFAULT_RECT_TAG,
        camera::DEFAULT_CAMERA_TAG
    },
    systems::rendering::draw::s_draw_by_tag
};

pub fn draw_frame(
    registry: &mut Registry,
) -> Result<(), Box<dyn std::error::Error>> {
    gl_enable(GLCapability::AlphaBlending);
    gl_blending_func(GLBlendingFactor::SrcAlpha, GLBlendingFactor::OneMinusSrcAlpha);

    let cameras = registry.get_entities_by_tag(DEFAULT_CAMERA_TAG);
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
