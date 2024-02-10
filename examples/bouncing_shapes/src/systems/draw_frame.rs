use quipi::{
    wrappers::opengl::{
        capabilities::*,
        draw::DrawMode,
    },
    systems::rendering::draw::s_draw_by_tag,
    Registry,
    schemas::entity2d::DEFAULT_RECT_TAG,
};

pub fn s_draw_frame(
    registry: &mut Registry
) -> Result<(), Box<dyn std::error::Error>> {
    gl_enable(GLCapability::AlphaBlending);
    gl_blending_func(GLBlendingFactor::SrcAlpha, GLBlendingFactor::OneMinusSrcAlpha);

    s_draw_by_tag(
        DEFAULT_RECT_TAG,
        registry,
        DrawMode::Triangles
    )?;

    Ok(())
}
