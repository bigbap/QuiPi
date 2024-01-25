use crate::Component;

/**
* the model portion for a Model View Projection matrix
*/
#[derive(Debug, Component, PartialEq, Clone)]
pub struct CTransform {
    pub translate: Option<glm::Vec3>,
    pub rotate: Option<Vec<(glm::Vec3, f32)>>,
    pub scale: Option<glm::Vec3>,
}

impl Default for CTransform {
    fn default() -> Self {
        Self {
            translate: Some(glm::vec3(0.0, 0.0, 0.0)),
            rotate: None,
            scale: None,
        }
    }
}
