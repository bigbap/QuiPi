use std::io;

#[derive(Debug, Default)]
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

#[derive(Debug, thiserror::Error)]
pub enum ObjectError {
    #[error("there was a problem reading from file")]
    ProblemReadingFile(
        #[from]
        #[source]
        io::Error
    ),

    #[error("there was a problem loading wavefront file")]
    ProblemLoadingWavefrontObj {
        #[from]
        #[source]
        from: tobj::LoadError,
    },

    #[error("there was a problem loading wavefront file")]
    ProblemLoadingGltf {
        #[from]
        #[source]
        from: gltf::Error,
    }
}

pub fn load_obj_file(
    full_path: String
) -> Result<(Vec<tobj::Model>, Vec<tobj::Material>), ObjectError> {
    println!("{full_path}");

    let (models, materials) = tobj::load_obj(
        full_path,
        &tobj::GPU_LOAD_OPTIONS
    )?;

    let materials = materials?;

    Ok((models, materials))
}

