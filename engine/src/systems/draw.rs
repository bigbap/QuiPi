use crate::{
    VersionedIndex,
    components::{
        CMesh,
        CViewMatrix,
        CProjectionMatrix,
        CModelMatrix,
        CMaterial, CRGBA
    },
    Registry,
    resources::shader::{
        Shader,
        UniformVariable
    },
    gfx::{
        draw::draw_ebo,
        texture
    },
    systems::material
};

/**
* draw entities by tag
*
* camera must have the following components:
* - CViewMatrix
* - CProjectionMatrix
*
* entities must have the following components:
* - CMesh
* - CModelMatrix
*
* shader must exist and be valid
*/
pub fn s_draw_by_tag(
    tag: &str,
    registry: &Registry,
    shader_id: &VersionedIndex,
    camera_id: &VersionedIndex,
) -> Result<(), Box<dyn std::error::Error>> {
    if let (Some(view), Some(projection), Some(shader)) = (
        registry.get_component::<CViewMatrix>(camera_id),
        registry.get_component::<CProjectionMatrix>(camera_id),
        registry.get_resource::<Shader>(shader_id)
    ) {
        let projection_view_matrix = projection.0 * view.0;

        let entities = registry.get_entities_by_tag(tag);
        for entity in entities.iter() {
            s_draw_entity(
                entity,
                registry,
                shader,
                &projection_view_matrix,
            );
        }
    }

    Ok(())
}

pub fn s_draw_entity(
    entity: &VersionedIndex,
    registry: &Registry,
    shader: &Shader,
    projection_view_matrix: &glm::Mat4,
) {
    // TODO: this can be optimized to have textures per tag instead of per entity
    bind_textures(entity, registry, shader);

    if let (Some(mesh), Some(model)) = (
        registry.get_component::<CMesh>(entity),
        registry.get_component::<CModelMatrix>(entity)
    ) {
        set_uniforms(
            entity,
            registry,
            shader,
            projection_view_matrix,
            &model.0
        );
        
        shader.program.use_program();
        mesh.mesh.vao.bind();
        draw_ebo(&mesh.mesh.vao);
        mesh.mesh.vao.unbind();
    }
}

fn set_uniforms(
    entity: &VersionedIndex,
    registry: &Registry,
    shader: &Shader,
    projection_view_matrix: &glm::Mat4,
    model: &glm::Mat4
) {
    shader.program.use_program();
    for uniform in shader.uniforms.iter() {
        match uniform {
            UniformVariable::Color(var) => set_color(entity, registry, shader, var),
            UniformVariable::MVPMatrix(var) => {
                let mvp_matrix = projection_view_matrix * model;
                
                shader.program.set_mat4(var, &mvp_matrix);
            },
            UniformVariable::ModelMatrix(var) => shader.program.set_mat4(var, model),
            UniformVariable::ViewMatrix(_var) => todo!(),
            UniformVariable::ProjectionMatrix(_var) => todo!(),
        }
    }
}

fn bind_textures(
    entity: &VersionedIndex,
    registry: &Registry,
    shader: &Shader
) {
    if let Some(mat) = registry.get_component::<CMaterial>(entity) {
        shader.program.use_program();
        shader.program.set_float(&format!("{}.shininess", mat.uniform_struct), mat.shininess);

        if let Some(diffuse) = material::s_get_texture(&mat.diffuse, registry) {
            shader.program.set_int(&format!("{}.diffuse", mat.uniform_struct), 0);
            texture::set_active_texture(0);
            texture::bind(&diffuse.id);
        }
        if let Some(specular) = material::s_get_texture(&mat.specular, registry) {
            shader.program.set_int(&format!("{}.specular", mat.uniform_struct), 1);
            texture::set_active_texture(1);
            texture::bind(&specular.id);
        }
    }
}

fn set_color(
    entity: &VersionedIndex,
    registry: &Registry,
    shader: &Shader,
    var: &str
) {
    if let Some(color) = registry.get_component::<CRGBA>(entity) {
        shader.program.set_float_3(var, (color.r, color.g, color.b));
    }
}
