use crate::{
    Component,
    systems::load_obj::ObjectConfig
};

#[derive(Debug, Component)]
pub struct CQuadConfig {
    pub width: f32,
    pub height: f32,
    pub center_x: f32,
    pub center_y: f32
}

impl CQuadConfig {
    pub fn to_obj_config(&self, color: (f32, f32, f32, f32)) -> ObjectConfig {
        let points: Vec<f32> = vec![
            self.center_x - (self.width / 2.0), self.center_y + (self.height / 2.0), 0.0, // top left
            self.center_x + (self.width / 2.0), self.center_y + (self.height / 2.0), 0.0, // top right
            self.center_x + (self.width / 2.0), self.center_y - (self.height / 2.0), 0.0, // bottom right
            self.center_x - (self.width / 2.0), self.center_y - (self.height / 2.0), 0.0 // bottom left
        ];
        let colors: Vec<f32> = vec![
            color.0, color.1, color.2, color.3,
            color.0, color.1, color.2, color.3,
            color.0, color.1, color.2, color.3,
            color.0, color.1, color.2, color.3,
        ];
        let indices = vec![
            0, 1, 2,
            3, 0, 2
        ];

        ObjectConfig {
            points,
            indices,
            colors,
            ..ObjectConfig::default()
        }
    }
}
