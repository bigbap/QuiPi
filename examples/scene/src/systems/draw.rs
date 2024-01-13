use engine::{
    VersionedIndex,
    Registry,
    gfx::{
        texture,
        draw::draw_ebo
    },
    resources::{
        Texture,
        Camera3D
    },
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
    let Some(camera) = registry.get_resource::<Camera3D>(&draw_cmp.camera) else { return Ok(()) };

    for (i, texture_i) in draw_cmp.textures.iter().enumerate() {
        let texture = registry.get_resource::<Texture>(texture_i).unwrap();

        texture::set_active_texture(i);
        texture::bind(&texture.index);
    }

    shader.program().use_program();
    mesh.vao().bind();

    let models = transforms.apply_transforms()?;
    for model in models {
        shader.program().set_mat4("model", &model);
        shader.program().set_mat4("view", &camera.get_view());
        shader.program().set_mat4("projection", &camera.get_projection());
        shader.program().set_float_3("viewPos", &camera.position);
        
        draw_ebo(&mesh.vao());
    }
    
    mesh.vao().unbind();

    Ok(())
}
