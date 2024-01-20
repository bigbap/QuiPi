pub mod material;
pub mod gizmo;
pub mod transform;
pub mod dimensions;
pub mod matrices;

pub use material::CMaterial;
pub use gizmo::CGizmo3D;
pub use transform::CTransform;
pub use dimensions::CDimensions;
pub use matrices::CModelMatrix;
pub use matrices::CViewMatrix;
pub use matrices::CProjectionMatrix;

use crate::{
    Registry,
    Component,
    gfx::ElementArrayMesh
};

/**
* 3D point in world space
*/
#[derive(Debug, Component, Clone, Copy, Default)]
pub struct CPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

/**
* 3D direction vector
*/
#[derive(Debug, Component, Clone, Copy, Default)]
pub struct CDirection {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

/**
* 3D velocity vector
*/
#[derive(Debug, Component, Clone, Copy)]
pub struct CVelocity {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

/**
* RGBA color
* (f32, f32, f32, f32)
*/
#[derive(Debug, Component)]
pub struct CRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

/**
* holds the VertexArray to draw a single mesh
*/
#[derive(Component, Debug)]
pub struct CMesh {
    pub mesh: ElementArrayMesh,
}

/**
* used to reduce the intensity of light over time
*
* https://wiki.ogre3d.org/tiki-index.php?page=-Point+Light+Attenuation
*/
#[derive(Debug, Component)]
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
#[derive(Debug, Component)]
pub struct CCutoff {
    pub inner_cutoff: f32,
    pub outer_cutoff: f32
}

/**
* common view settings, used for cameras
*/
#[derive(Debug, Component)]
pub struct CViewSettings {
    pub fov: f32,
    pub aspect_ratio: f32
}

/**
* https://en.wikipedia.org/wiki/Euler_angles
*/
#[derive(Debug, Component, Default)]
pub struct CEulerAngles {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32
}

/**
* near and far planes used for clipping
*/
#[derive(Debug, Component)]
pub struct CZPlanes {
    pub near_plane: f32,
    pub far_plane: f32
}

pub fn register_components(registry: &mut Registry) {
    registry
        .register_component::<CAttenuation>()
        .register_component::<CRGBA>()
        .register_component::<CCutoff>()
        .register_component::<CDimensions>()
        .register_component::<CDirection>()
        .register_component::<CEulerAngles>()
        .register_component::<CGizmo3D>()
        .register_component::<CMaterial>()
        .register_component::<CMesh>()
        .register_component::<CModelMatrix>()
        .register_component::<CTransform>()
        .register_component::<CPosition>()
        .register_component::<CProjectionMatrix>()
        .register_component::<CVelocity>()
        .register_component::<CViewMatrix>()
        .register_component::<CViewSettings>()
        .register_component::<CZPlanes>();
}
