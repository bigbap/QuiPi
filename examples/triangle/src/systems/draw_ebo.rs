use ::engine::{
    VersionedIndex,
    Registry,
    components::MeshComponent,
    gfx
};

use crate::{
    components::DrawComponent,
    resources::Shader
};

pub fn draw_ebo(
    entity: &VersionedIndex,
    registry: &mut Registry
) -> Result<(), Box<dyn std::error::Error>> {
    let Some(shader_cmp) = registry.get_component::<DrawComponent>(entity) else { return Ok(()) };
    let Some(shader) = registry.get_resource::<Shader>(&shader_cmp.shader_id) else { return Ok(()) };
    let Some(mesh) = registry.get_component::<MeshComponent>(entity) else { return Ok(()) };
    
    shader.program().use_program();

    mesh.vao().bind();
    gfx::draw::draw_ebo(mesh.vao());
    mesh.vao().unbind();

    Ok(())
}
