use crate::Component;

#[derive(Debug, Component, Default)]
pub struct CModelMatrix(pub glm::Mat4);

#[derive(Debug, Component, Default)]
pub struct CViewMatrix(pub glm::Mat4);

#[derive(Debug, Component, Default)]
pub struct CProjectionMatrix(pub glm::Mat4);
