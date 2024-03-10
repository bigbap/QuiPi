use crate::resources::Resource;

#[derive(Resource)]
pub struct ClearColor(pub f32, pub f32, pub f32, pub f32);

impl ClearColor {
    pub fn new(color: (f32, f32, f32, f32)) -> Self {
        Self(color.0, color.1, color.2, color.3)
    }

    pub fn as_tuple(&self) -> (f32, f32, f32, f32) {
        (self.0, self.1, self.2, self.3)
    }
}
