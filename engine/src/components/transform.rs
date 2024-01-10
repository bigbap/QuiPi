use crate::Component;

#[derive(Debug, Default, Component, PartialEq)]
pub struct TransformComponent {
    pub translate: Option<glm::Vec3>,
    pub rotate: Option<glm::Vec3>,
    pub scale: Option<glm::Vec3>,

    pub angle: Option<f32>
}
