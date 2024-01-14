use engine::{
    VersionedIndex,
    Registry,
    gfx::{
        texture,
        draw::draw_ebo
    },
    resources::Camera3D,
    components::TransformComponent
};

use crate::{
    components::{
        DrawComponent,
        MeshComponent
    },
    resources::Shader
};

pub fn draw(
    entity: &VersionedIndex,
    registry: &Registry,
) -> Result<(), Box<dyn std::error::Error>> {
    let Some(draw_cmp) = registry.get_component::<DrawComponent>(entity) else { return Ok(()) };
    let Some(mesh) = registry.get_component::<MeshComponent>(entity) else { return Ok(()) };
    let Some(transforms) = registry.get_component::<TransformComponent>(entity) else { return Ok(()) };

    let Some(shader) = registry.get_resource::<Shader>(&draw_cmp.shader_id) else { return Ok(()) };
    let Some(camera) = registry.get_resource::<Camera3D>(&draw_cmp.camera_id) else { return Ok(()) };

    for material in draw_cmp.materials.iter() {
        if let Some(diffuse) = material.get_texture(&material.diffuse, registry) {
            texture::set_active_texture(diffuse.index);
            texture::bind(&diffuse.id);
        }
        if let Some(specular) = material.get_texture(&material.specular, registry) {
            texture::set_active_texture(specular.index);
            texture::bind(&specular.id);
        }
    }

    if let Some(color) = draw_cmp.color {
        shader.program().set_float_3("color", color);
    }

    shader.program().use_program();
    mesh.vao().bind();

    let models = transforms.apply_transforms()?;
    for model in models {
        shader.program().set_mat4("model", &model);
        shader.program().set_mat4("view", &camera.get_view());
        shader.program().set_mat4("projection", &camera.get_projection());
        shader.program().set_float_3(
            "viewPos",
            (camera.position.x, camera.position.y, camera.position.z)
        );
        
        draw_ebo(mesh.vao());
    }
    
    mesh.vao().unbind();

    Ok(())
}
