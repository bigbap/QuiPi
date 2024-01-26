use crate::Component;

/**
* the model portion for a Model View Projection matrix
*/
#[derive(Debug, Component, PartialEq, Clone)]
pub struct CTransform {
    pub translate: glm::Vec3,
    pub rotate: Option<glm::Vec3>,
    pub scale: Option<glm::Vec3>,

    pub angle: f32
}

impl Default for CTransform {
    fn default() -> Self {
        Self {
            translate: glm::vec3(0.0, 0.0, 0.0),
            rotate: None,
            scale: None,

            angle: 0.0
        }
    }
}

impl CTransform {
    /**
    * Transformations do not commute, so it's important to know
    * the order that is being used here.
    *
    * Transformations are done in this order:
    * 1. translate
    * 2. rotate
    * 3. scale
    */
    pub fn to_matrix(&self) -> glm::Mat4 {
        let matrix = glm::Mat4::identity();
        let matrix = glm::translate(&matrix, &self.translate);
        let matrix = match &self.rotate {
            None => matrix,
            Some(rotate) => {
                // TODO: change this to use quaternions.
                glm::rotate(&matrix, self.angle, &glm::normalize(&rotate))
            }
        };
        match self.scale {
            Some(scale) => glm::scale(&matrix, &scale),
            None => matrix
        }
    }
}
