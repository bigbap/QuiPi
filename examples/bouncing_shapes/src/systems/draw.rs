use engine::{
    VersionedIndex,
    components::CMesh,
    Registry,
    resources::Shader,
    systems::mvp_matrices::*,
    gfx::draw::draw_ebo
};

use crate::{WIDTH, HEIGHT};

const MODEL: &str = "model";
const VIEW: &str = "view";
const PROJECTION: &str = "projection";

pub fn s_draw(
    registry: &mut Registry,
    shader_id: &VersionedIndex,
    camera_id: &VersionedIndex
) -> Result<(), Box<dyn std::error::Error>> {
    let entities = registry.get_entities_by_tag("quad");

    for entity in entities.iter() {
        let Some(cmp_mesh) = registry.get_component::<CMesh>(entity) else { return Ok(()) };
        let Some(shader) = registry.get_resource::<Shader>(shader_id) else { return Ok(()) };

        let model = s_model_matrix_3d(entity, registry);
        let view = s_view_matrix_3d(camera_id, registry);
        let projection = s_ortho_projection_matrix_3d(camera_id, registry);

        // println!("model view: {:?}", view * model);
        // println!("model view projection: {:?}", projection * view * model * glm::vec4(1.0, 1.0, 0.0, 1.0));

        shader.program().use_program();
        cmp_mesh.mesh.vao.bind();

        shader.program().set_mat4(MODEL, &model);
        shader.program().set_mat4(VIEW, &view);
        shader.program().set_mat4(PROJECTION, &projection);

        draw_ebo(&cmp_mesh.mesh.vao);

        cmp_mesh.mesh.vao.unbind();
    }

    Ok(())
}

