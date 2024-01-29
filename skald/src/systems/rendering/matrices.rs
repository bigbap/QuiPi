use crate::components::{
    CTransform,
    CGizmo3D,
};

pub fn view_matrix(
    transform: &CTransform,
    gizmo: &CGizmo3D
) -> glm::Mat4 {
    let position = glm::vec3(
        transform.translate.x,
        transform.translate.y,
        transform.translate.z
    );

    glm::look_at(
        &position, 
        &(position + gizmo.front),
        &gizmo.up
    )
}