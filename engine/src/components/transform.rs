use crate::Component;

/**
* the model portion for a Model View Projection matrix
*/
#[derive(Debug, Component, PartialEq, Clone)]
pub struct CTransform {
    pub translate: Option<glm::Vec3>,
    pub rotate: Option<glm::Vec3>,
    pub scale: Option<glm::Vec3>,

    pub angle: f32
}

impl Default for CTransform {
    fn default() -> Self {
        Self {
            translate: Some(glm::vec3(0.0, 0.0, 0.0)),
            rotate: None,
            scale: None,
            angle: 0.0
        }
    }
}
