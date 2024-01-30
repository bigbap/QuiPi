use quipi::{
    facades::{
        freetype::TextRenderer,
        opengl::{
            capabilities::*,
            draw::DrawMode,
        },
    },
    systems::rendering::{
        Renderer2D,
        canvas,
    },
    Registry,
    VersionedIndex
};

// use crate::{WIDTH, HEIGHT};

pub fn s_draw_frame(
    registry: &mut Registry,
    shader: &VersionedIndex,
    renderer: &Renderer2D,
    text_renderer: &TextRenderer
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

    let entity_count = registry.entity_count();
    text_renderer.draw(
        format!("entities: {}", entity_count),
        glm::vec2(width as f32 - 120.0, height as f32 - 30.0)
    );

    Ok(())
}
