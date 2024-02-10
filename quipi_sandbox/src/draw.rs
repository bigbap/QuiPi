use quipi::{
    schemas::entity2d::DEFAULT_RECT_TAG,
    systems::rendering::draw::s_draw_by_tag,
    wrappers::opengl::{
        capabilities::{
            gl_blending_func,
            gl_enable,
            GLBlendingFactor,
            GLCapability
        },
        draw::DrawMode
    },
    Registry
};

pub fn draw_frame(
    registry: &mut Registry,
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
