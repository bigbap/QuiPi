use crate::Component;

#[derive(Debug, Component)]
pub struct CModelMatrix(pub glm::Mat4);

impl Default for CModelMatrix {
    fn default() -> Self {
        Self(glm::Mat4::identity())
    }
}

#[derive(Debug, Component)]
pub struct CViewMatrix(pub glm::Mat4);

impl Default for CViewMatrix {
    fn default() -> Self {
        Self(glm::Mat4::identity())
    }
}
