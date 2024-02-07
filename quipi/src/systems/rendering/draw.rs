use crate::{
    VersionedIndex,
    components::{
        CBoundingBox,
        CCamera,
        CMaterial,
        CModelMatrix,
        CViewMatrix,
        CRGBA,
        CShader,
        CMesh, CTag
    },
    Registry,
    resources::shader::{
        RShader,
        UniformVariable
    },
    systems::material,
    wrappers::opengl::{
        self,
        draw::*
    }
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
    camera_id: &VersionedIndex,
    mode: DrawMode
) -> Result<(), Box<dyn std::error::Error>> {
    let entities = registry.entities.query::<CTag>(CTag { tag: tag.to_string() });

    for entity in entities.iter() {
        if let Some(shader_id) = registry.entities.get::<CShader>(&entity) {
            s_draw_entity(
                &entity,
                registry,
                camera_id,
                shader_id,
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
    shader: &CShader,
    mode: DrawMode
) {
    if let Some(shader) = registry.resources.get::<RShader>(&shader.shader) {
        // TODO: this can be optimized to have textures per tag instead of per entity
        bind_textures(entity, registry, shader);

        if let Some(mesh) = registry.entities.get::<CMesh>(entity) {
            set_uniforms(
                entity,
                registry,
                shader,
                camera
            );

            draw_node(mesh, shader, mode);
        }
    }
}

fn draw_node(
    node: &CMesh,
    shader: &RShader,
    mode: DrawMode
) {
    if let Some(mesh) = &node.mesh {
        shader.program.use_program();
        mesh.vao.bind();
        opengl::draw::gl_draw(
            match mesh.ebo {
                Some(_) => DrawBuffer::Elements,
                _ => DrawBuffer::Arrays
            },
            mode,
            mesh.vao.count()
        );
        mesh.vao.unbind();
    }
}

fn set_uniforms(
    entity: &VersionedIndex,
    registry: &Registry,
    shader: &RShader,
    camera: &VersionedIndex,
) {
    shader.program.use_program();
    for uniform in shader.uniforms.iter() {
        match uniform {
            UniformVariable::Color(var) => set_color(entity, registry, shader, var),
            UniformVariable::MVPMatrix(var) => {
                if let (Some(model), Some(view), Some(c_camera)) = (
                    registry.entities.get::<CModelMatrix>(entity),
                    registry.entities.get::<CViewMatrix>(camera),
                    registry.entities.get::<CCamera>(camera),
                ) {
                    let mvp_matrix = c_camera.projection * view.0 * model.0;

                    shader.program.set_mat4(var, &mvp_matrix);
                }
            },
            UniformVariable::ModelMatrix(var) => {
                if let Some(model) = registry.entities.get::<CModelMatrix>(entity) {
                    shader.program.set_mat4(var, &model.0)
                }
            },
            UniformVariable::ViewMatrix(var) => {
                if let Some(view) = registry.entities.get::<CViewMatrix>(camera) {
                    shader.program.set_mat4(var, &view.0)
                }
            },
            UniformVariable::ProjectionMatrix(var) => {
                if let Some(c_camera) = registry.entities.get::<CCamera>(camera) {
                    shader.program.set_mat4(var, &c_camera.projection)
                }
            },
            UniformVariable::NearPlane(var) => {
                if let Some(b_box) = registry.entities.get::<CBoundingBox>(camera) {
                    shader.program.set_float(var, b_box.near)
                }
            },
            UniformVariable::FarPlane(var) => {
                if let Some(b_box) = registry.entities.get::<CBoundingBox>(camera) {
                    shader.program.set_float(var, b_box.far)
                }
            },
        }
    }
}

fn bind_textures(
    entity: &VersionedIndex,
    registry: &Registry,
    shader: &RShader
) {
    if let Some(mat) = registry.entities.get::<CMaterial>(entity) {
        shader.program.use_program();
        shader.program.set_float(&format!("{}.shininess", mat.uniform_struct), mat.shininess);

        if let Some(diffuse) = material::s_get_texture(&mat.diffuse, registry) {
            shader.program.set_int(&format!("{}.diffuse", mat.uniform_struct), 0);
            diffuse.0.use_texture(0);
        }
        if let Some(specular) = material::s_get_texture(&mat.specular, registry) {
            shader.program.set_int(&format!("{}.specular", mat.uniform_struct), 1);
            specular.0.use_texture(1);
        }
    }
}

fn set_color(
    entity: &VersionedIndex,
    registry: &Registry,
    shader: &RShader,
    var: &str
) {
    if let Some(color) = registry.entities.get::<CRGBA>(entity) {
        shader.program.set_float_3(var, (color.r, color.g, color.b));
    }
}
