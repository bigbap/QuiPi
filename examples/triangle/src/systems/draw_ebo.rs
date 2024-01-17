use ::engine::{
    VersionedIndex,
    Registry,
    components::CMesh,
    resources::Shader,
    gfx
};

pub fn draw_ebo(
    entity: &VersionedIndex,
    registry: &mut Registry,
    shader_id: &VersionedIndex
) -> Result<(), Box<dyn std::error::Error>> {
    let Some(shader) = registry.get_resource::<Shader>(shader_id) else { return Ok(()) };
    let Some(mesh) = registry.get_component::<CMesh>(entity) else { return Ok(()) };
    
    shader.program().use_program();

    mesh.mesh.vao.bind();
    gfx::draw::draw_ebo(&mesh.mesh.vao);
    mesh.mesh.vao.unbind();

    Ok(())
}
