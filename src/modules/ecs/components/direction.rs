/**
* 3D direction vector
*/
#[derive(Debug, Component, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct CDirection {
    pub x: f32,
    pub y: f32,
    pub z: f32
}