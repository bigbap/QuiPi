use engine::{
    VersionedIndex,
    Registry,
    gfx::{
        texture,
        draw::draw_ebo, ShaderProgram
    },
    resources::Camera3D,
    components::ModelTransform
};

use crate::{
    components::{
        Draw,
        Mesh,
        Color
    },
    resources::Shader
};

const COLOR_VARIABLE: &str = "color";
const MODEL: &str = "model";
const VIEW: &str = "view";
const PROJECTION: &str = "projection";
const VIEW_POS: &str = "viewPos";

pub fn draw(
    entity: &VersionedIndex,
    registry: &Registry,
) -> Result<(), Box<dyn std::error::Error>> {
    let Some(cmp_draw) = registry.get_component::<Draw>(entity) else { return Ok(()) };
    let Some(cmp_mesh) = registry.get_component::<Mesh>(entity) else { return Ok(()) };
    let Some(cmp_transforms) = registry.get_component::<ModelTransform>(entity) else { return Ok(()) };

    let Some(shader) = registry.get_resource::<Shader>(&cmp_draw.shader_id) else { return Ok(()) };
    let Some(camera) = registry.get_resource::<Camera3D>(&cmp_draw.camera_id) else { return Ok(()) };

    set_color(entity, registry, shader.program());
    bind_textures(cmp_draw, registry);

    shader.program().use_program();
    cmp_mesh.vao().bind();

    let models = cmp_transforms.apply_transforms()?;
    for model in models {
        shader.program().set_mat4(MODEL, &model);
        shader.program().set_mat4(VIEW, &camera.get_view());
        shader.program().set_mat4(PROJECTION, &camera.get_projection());
        shader.program().set_float_3(VIEW_POS, camera.position_tup());
        
        draw_ebo(cmp_mesh.vao());
    }
    
    cmp_mesh.vao().unbind();

    Ok(())
}

fn bind_textures(
    cmp_draw: &Draw,
    registry: &Registry
) {
    for material in cmp_draw.materials.iter() {
        if let Some(diffuse) = material.get_texture(&material.diffuse, registry) {
            texture::set_active_texture(diffuse.index);
            texture::bind(&diffuse.id);
        }
        if let Some(specular) = material.get_texture(&material.specular, registry) {
            texture::set_active_texture(specular.index);
            texture::bind(&specular.id);
        }
    }
}

fn set_color(
    entity: &VersionedIndex,
    registry: &Registry,
    shader: &ShaderProgram
) {
    if let Some(color) = registry.get_component::<Color>(entity) {
        shader.set_float_3(COLOR_VARIABLE, (color.0, color.1, color.2));
    }
}
