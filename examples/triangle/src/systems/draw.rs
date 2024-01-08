use engine::{
    VersionedIndex,
    Registry,
    gfx::MeshComponent
};

use crate::{
    components::DrawComponent,
    resources::Shader
};

pub fn draw(
    entity: &VersionedIndex,
    registry: &mut Registry
) -> Result<(), Box<dyn std::error::Error>> {
    let Some(shader_id) = registry.get_component::<DrawComponent>(entity) else { return Ok(()) };
    let shader_id = shader_id.shader_id;

    let Some(shader) = registry.get_resource::<Shader>(&shader_id) else { return Ok(()) };
    shader.program().use_program();

    let Some(mesh) = registry.get_component::<MeshComponent>(entity) else { return Ok(()) };
    mesh.vao().bind();

    unsafe {
        gl::DrawElements(
            gl::TRIANGLES,
            mesh.vao().count(),
            gl::UNSIGNED_INT,
            std::ptr::null()
        );
    }

    mesh.vao().unbind();

    Ok(())
}
