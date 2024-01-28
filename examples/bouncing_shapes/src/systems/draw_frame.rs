use skald::{
    Registry,
    VersionedIndex,
    systems::draw::{
        DrawMode,
        s_draw_by_tag
    },
    core::text::TextRenderer, gl_capabilities::{self, GLCapability, BlendingFactor}
};

use crate::{WIDTH, HEIGHT};

pub fn s_draw_frame(
    registry: &Registry,
    shader: &VersionedIndex,
    camera: &VersionedIndex,
    text_renderer: &TextRenderer
) -> Result<(), Box<dyn std::error::Error>> {
    gl_capabilities::enable(GLCapability::AlphaBlending);
    gl_capabilities::blending_func(BlendingFactor::SrcAlpha, BlendingFactor::OneMinusSrcAlpha);

    s_draw_by_tag(
        "quad",
        registry,
        shader,
        camera,
        DrawMode::Triangles
    )?;

    let entity_count = registry.entity_count();
    text_renderer.draw(
        format!("entities: {}", entity_count),
        glm::vec2(WIDTH as f32 - 120.0, HEIGHT as f32 - 30.0)
    );

    Ok(())
}
