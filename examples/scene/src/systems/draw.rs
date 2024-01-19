use engine::{
    VersionedIndex,
    Registry,
    gfx::{
        texture,
        draw::draw_ebo, ShaderProgram
    },
    components::CPosition,
    systems::mvp_matrices::{
        s_view_matrix_3d,
        s_model_matrix_3d,
        s_projection_matrix_3d
    },
    systems::material
};

use crate::{
    components::{
        CMesh,
        CRGBA,
        CMaterial
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
    shader_id: &VersionedIndex,
    camera: &VersionedIndex
) -> Result<(), Box<dyn std::error::Error>> {
    let Some(cmp_mesh) = registry.get_component::<CMesh>(entity) else { return Ok(()) };
    let Some(pos) = registry.get_component::<CPosition>(entity) else { return Ok(()) };
    let Some(camera_position) = registry.get_component::<CPosition>(camera) else { return Ok(()) };

    let Some(shader) = registry.get_resource::<Shader>(shader_id) else { return Ok(()) };

    set_color(entity, registry, shader.program());
    bind_textures(entity, registry, shader.program());

    shader.program().use_program();
    cmp_mesh.mesh.vao.bind();
    
    let model = s_model_matrix_3d(entity, registry, pos);
    shader.program().set_mat4(MODEL, &model);
    shader.program().set_mat4(VIEW, &s_view_matrix_3d(camera, registry));
    shader.program().set_mat4(PROJECTION, &s_projection_matrix_3d(camera, registry));
    shader.program().set_float_3(VIEW_POS, (camera_position.x, camera_position.y, camera_position.z));

    draw_ebo(&cmp_mesh.mesh.vao);
    
    cmp_mesh.mesh.vao.unbind();

    Ok(())
}

fn bind_textures(
    entity: &VersionedIndex,
    registry: &Registry,
    shader: &ShaderProgram
) {
    let mat = registry.get_component::<CMaterial>(entity).unwrap();

    shader.use_program();
    shader.set_float("material.shininess", 0.6 * 128.0);

    if let Some(diffuse) = material::s_get_texture(&mat.diffuse, registry) {
        shader.set_int("material.diffuse", 0);
        texture::set_active_texture(0);
        texture::bind(&diffuse.id);
    }
    if let Some(specular) = material::s_get_texture(&mat.specular, registry) {
        shader.set_int("material.specular", 1);
        texture::set_active_texture(1);
        texture::bind(&specular.id);
    }
}

fn set_color(
    entity: &VersionedIndex,
    registry: &Registry,
    shader: &ShaderProgram
) {
    if let Some(color) = registry.get_component::<CRGBA>(entity) {
        shader.set_float_3(COLOR_VARIABLE, (color.r, color.g, color.b));
    }
}
