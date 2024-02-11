use serde::{Serialize, Deserialize};

pub mod obj_loader;
pub mod gltf_loader;
pub mod image;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ObjectConfig {
    pub points: Vec<f32>,
    pub normals: Vec<f32>,
    pub texture_coords: Vec<f32>,
    pub colors: Vec<f32>,
    pub indices: Vec<u32>,
    pub material_id: usize
    // pub materials: Option<Vec<tobj::Material>>
}

impl ObjectConfig {
    pub fn from_obj(
        models: Vec<tobj::Model>
    ) -> Result<Vec<ObjectConfig>, Box<dyn std::error::Error>> {
        let mut obj_configs = Vec::<ObjectConfig>::new();

        for model in models {
            let material_id = model.mesh.material_id.unwrap();
            let points = model.mesh.positions;
            let normals = model.mesh.normals;
            let texture_coords = model.mesh.texcoords;
            let colors = model.mesh.vertex_color;
            let indices = model.mesh.indices;

            obj_configs.push(ObjectConfig {
                points,
                indices,
                normals,
                texture_coords,
                colors,
                material_id
            });
        }

        Ok(obj_configs)
    }
}
