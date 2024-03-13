use crate::assets::Asset;

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    pub indices: Vec<u32>,
    pub vertices: Vec<glm::Vec4>,
}

impl Asset for Mesh {}
impl Mesh {
    pub fn new() -> Self {
        let indices = vec![0, 1, 3, 1, 2, 3];
        let vertices = vec![
            glm::vec4(1.0, 1.0, 0.0, 1.0),
            glm::vec4(1.0, -1.0, 0.0, 1.0),
            glm::vec4(-1.0, -1.0, 0.0, 1.0),
            glm::vec4(-1.0, 1.0, 0.0, 1.0),
        ];

        Self { indices, vertices }
    }
}

// pub struct Quad {
//     pub width: f32,
//     pub height: f32,
//     pub center_x: f32,
//     pub center_y: f32,
// }

// impl AssetLoader for Quad {
//     type AssetType = Mesh;

//     fn load(&mut self) -> crate::QPResult<Mesh> {
//         let indices = vec![0, 1, 3, 1, 2, 3];
//         let vertices = vec![
//             glm::vec4(1.0, 1.0, 0.0, 1.0),
//             glm::vec4(1.0, -1.0, 0.0, 1.0),
//             glm::vec4(-1.0, -1.0, 0.0, 1.0),
//             glm::vec4(-1.0, 1.0, 0.0, 1.0),
//         ];

//         Ok(Mesh { indices, vertices })
//     }
// }
