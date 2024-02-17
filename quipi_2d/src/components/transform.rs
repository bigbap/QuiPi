use quipi_core::Component;
use serde::{Deserialize, Serialize};

/**
* the model portion for a Model View Projection matrix
*/
#[derive(Debug, Component, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct CTransform2D {
    pub translate: glm::Vec2,
    pub rotate: f32, // rotation on happens on z-axis
    pub scale: glm::Vec2,
}

impl Default for CTransform2D {
    fn default() -> Self {
        Self {
            translate: glm::vec2(0.0, 0.0,),
            rotate: 0.0,
            scale: glm::vec2(1.0, 1.0),
        }
    }
}

impl CTransform2D {
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
        let translate = glm::vec3(self.translate.x, self.translate.y, 0.0);
        let rotate = glm::vec3(0.0, 0.0, 1.0);
        let scale = glm::vec3(self.scale.x, self.scale.y, 0.0);

        let matrix = glm::Mat4::identity();
        let matrix = glm::translate(&matrix, &translate);
        let matrix = glm::rotate(&matrix, self.rotate, &rotate);
        
        glm::scale(&matrix, &scale)
    }
}