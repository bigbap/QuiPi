use engine::Component;

#[derive(Debug, Default, Component, PartialEq)]
pub struct TransformComponent {
    pub translate: glm::Vec3,
    pub rotate: glm::Vec3,
    pub scale: glm::Vec3,
}
