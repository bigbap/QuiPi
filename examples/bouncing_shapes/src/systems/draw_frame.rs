use quipi::{
    wrappers::opengl::{
        capabilities::*,
        draw::DrawMode,
    },
    systems::rendering::draw::s_draw_by_tag,
    Registry,
    VersionedIndex,
};

pub fn s_draw_frame(
    registry: &mut Registry,
    camera: &VersionedIndex
) -> Result<(), Box<dyn std::error::Error>> {
    gl_enable(GLCapability::AlphaBlending);
    gl_blending_func(GLBlendingFactor::SrcAlpha, GLBlendingFactor::OneMinusSrcAlpha);

    s_draw_by_tag(
        "rect",
        registry,
        camera,
        DrawMode::Triangles
    )?;

    Ok(())
}
