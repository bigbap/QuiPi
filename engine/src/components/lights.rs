use crate::Component;
// https://learnopengl.com/Lighting/Light-casters

use crate::resources::Material;

#[derive(Debug, Component)]
pub struct LightComponent {
    pub direction: (f32, f32, f32),
    pub position: (f32, f32, f32),
    pub material: Material,
    
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
    
    /**
    * outer_cuttoff and inner_cuttoff is to provide smoothness at the edges of a spotlight
    */
    pub outer_cutoff: f32,
    pub inner_cutoff: f32
}

