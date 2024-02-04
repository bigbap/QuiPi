pub mod bounding_box;
pub mod camera;
pub mod distance;
pub mod drawable;
pub mod gizmo;
pub mod material;
pub mod matrices;
pub mod model;
pub mod shapes;
pub mod speed;
pub mod states;
pub mod target;
pub mod transform;
pub mod resources;
pub mod unique_id;

pub use resources::*;

pub use bounding_box::CBoundingBox;
pub use camera::CCamera;
pub use drawable::CMesh;
pub use drawable::CShader;
pub use material::CMaterial;
pub use gizmo::CGizmo3D;
pub use transform::CTransform;
pub use distance::CDistance;
pub use matrices::CModelMatrix;
pub use matrices::CViewMatrix;
pub use model::CModelNode;
pub use shapes::CRect;
pub use shapes::CShape;
pub use speed::CSpeed;
pub use states::CMouseBtnState;
pub use target::CTarget;
pub use unique_id::CUniqueId;

use crate::{
    Registry,
    Component,
};
use serde::{Deserialize, Serialize};

/**
* 3D direction vector
*/
#[derive(Debug, Component, Clone, Copy, Default, Serialize, Deserialize)]
pub struct CDirection {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

/**
* 3D velocity vector
*/
#[derive(Debug, Component, Clone, Copy, Default, Serialize, Deserialize)]
pub struct CVelocity {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

/**
* RGBA color
* (f32, f32, f32, f32)
*/
#[derive(Debug, Component, Default, Serialize, Deserialize)]
pub struct CRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

/**
* used to reduce the intensity of light over time
*
* https://wiki.ogre3d.org/tiki-index.php?page=-Point+Light+Attenuation
*/
#[derive(Debug, Component, Default, Serialize, Deserialize)]
pub struct CAttenuation {
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

/**
* used to smooth the edges around a spot light
*
* https://learnopengl.com/Lighting/Light-casters
*/
#[derive(Debug, Component, Default, Serialize, Deserialize)]
pub struct CCutoff {
    pub inner_cutoff: f32,
    pub outer_cutoff: f32
}

/**
* https://en.wikipedia.org/wiki/Euler_angles
*/
#[derive(Debug, Component, Default, Serialize, Deserialize)]
pub struct CEulerAngles {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32
}

pub fn register_components(registry: &mut Registry) {
    registry
        .register_component::<CCamera>()
        .register_component::<CAttenuation>()
        .register_component::<CBoundingBox>()
        .register_component::<CRGBA>()
        .register_component::<CCutoff>()
        .register_component::<CDirection>()
        .register_component::<CDistance>()
        .register_component::<CEulerAngles>()
        .register_component::<CGizmo3D>()
        .register_component::<CMaterial>()
        .register_component::<CMesh>()
        .register_component::<CModelMatrix>()
        .register_component::<CModelNode>()
        .register_component::<CMouseBtnState>()
        .register_component::<CSpeed>()
        .register_component::<CTransform>()
        .register_component::<CRect>()
        .register_component::<CShader>()
        .register_component::<CShape>()
        .register_component::<CTarget>()
        .register_component::<CVelocity>()
        .register_component::<CViewMatrix>()
        .register_component::<CUniqueId>();
}
