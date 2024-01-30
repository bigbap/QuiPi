use quipi::{
    wrappers::opengl::{
        capabilities::*,
        draw::DrawMode,
    },
    systems::rendering::{
        Renderer2D,
        canvas,
    },
    Registry,
    VersionedIndex
};

pub fn s_draw_frame(
    registry: &mut Registry,
    shader: &VersionedIndex,
    renderer: &Renderer2D,
) -> Result<(), Box<dyn std::error::Error>> {
    gl_enable(GLCapability::AlphaBlending);
    gl_blending_func(GLBlendingFactor::SrcAlpha, GLBlendingFactor::OneMinusSrcAlpha);

    let (x, y, width, height) = canvas::get_dimensions();
    renderer.update_projection_matrix(
        registry,
        Some(x as f32),
        Some(width as f32),
        Some(y as f32),
        Some(height as f32),
    );

    renderer.draw_by_tag(
        "quad",
        registry,
        shader,
        DrawMode::Triangles
    )?;

    Ok(())
}
