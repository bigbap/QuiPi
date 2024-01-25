use crate::{
    VersionedIndex,
    components::{
        CViewMatrix,
        CProjectionMatrix,
        CModelMatrix,
        CMaterial,
        CRGBA,
        CModelNode, CZPlanes,
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

pub use crate::gfx::draw::DrawMode;

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
    mode: DrawMode
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(shader) = registry.get_resource::<Shader>(shader_id) {
        let entities = registry.get_entities_by_tag(tag);
        for entity in entities.iter() {
            s_draw_entity(
                entity,
                registry,
                camera_id,
                shader,
                mode
            );
        }
    }

    Ok(())
}

pub fn s_draw_entity(
    entity: &VersionedIndex,
    registry: &Registry,
    camera: &VersionedIndex,
    shader: &Shader,
    mode: DrawMode
) {
    // TODO: this can be optimized to have textures per tag instead of per entity
    bind_textures(entity, registry, shader);

    if let Some(root) = registry.get_component::<CModelNode>(entity) {
        set_uniforms(
            entity,
            registry,
            shader,
            camera
        );
    
        draw_node(root, shader, mode);
    }
}

fn draw_node(
    node: &CModelNode,
    shader: &Shader,
    mode: DrawMode
) {
    if let Some(mesh) = &node.mesh {
        shader.program.use_program();
        mesh.vao.bind();
        draw_ebo(&mesh.vao, mode);
        mesh.vao.unbind();
    }
}

fn set_uniforms(
    entity: &VersionedIndex,
    registry: &Registry,
    shader: &Shader,
    camera: &VersionedIndex,
) {
    shader.program.use_program();
    for uniform in shader.uniforms.iter() {
        match uniform {
            UniformVariable::Color(var) => set_color(entity, registry, shader, var),
            UniformVariable::MVPMatrix(var) => {
                if let (Some(model), Some(view), Some(projection)) = (
                    registry.get_component::<CModelMatrix>(entity),
                    registry.get_component::<CViewMatrix>(camera),
                    registry.get_component::<CProjectionMatrix>(camera),
                ) {
                    let mvp_matrix = projection.0 * view.0 * model.0;

                    shader.program.set_mat4(var, &mvp_matrix);
                }
            },
            UniformVariable::ModelMatrix(var) => {
                if let Some(model) = registry.get_component::<CModelMatrix>(entity) {
                    shader.program.set_mat4(var, &model.0)
                }
            },
            UniformVariable::ViewMatrix(var) => {
                if let Some(view) = registry.get_component::<CViewMatrix>(camera) {
                    shader.program.set_mat4(var, &view.0)
                }
            },
            UniformVariable::ProjectionMatrix(var) => {
                if let Some(projection) = registry.get_component::<CProjectionMatrix>(camera) {
                    shader.program.set_mat4(var, &projection.0)
                }
            },
            UniformVariable::NearPlane(var) => {
                if let Some(z_planes) = registry.get_component::<CZPlanes>(camera) {
                    shader.program.set_float(var, z_planes.near_plane)
                }
            },
            UniformVariable::FarPlane(var) => {
                if let Some(z_planes) = registry.get_component::<CZPlanes>(camera) {
                    shader.program.set_float(var, z_planes.far_plane)
                }
            },
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
