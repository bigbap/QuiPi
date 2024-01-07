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
    registry: &Registry
) -> Result<(), Box<dyn std::error::Error>> {
    let mut reg_cmp = registry.components.borrow_mut();
    let mut reg_res = registry.resources.borrow_mut();

    let Some(mesh) = reg_cmp.get_component::<MeshComponent>(entity) else { return Ok(()) };
    mesh.vao().bind();

    let Some(shader_id) = reg_cmp.get_component::<DrawComponent>(entity) else { return Ok(()) };
    let Some(shader) = reg_res.get_component::<Shader>(&shader_id.shader_id) else { return Ok(()) };

    shader.program().use_program();

    let Some(mesh) = reg_cmp.get_component::<MeshComponent>(entity) else { return Ok(()) };

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
