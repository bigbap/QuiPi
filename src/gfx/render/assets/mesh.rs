use crate::assets::{Asset, AssetLoader};

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    pub indices: Vec<u32>,
    pub vertices: Vec<glm::Vec4>,
}

impl Asset for Mesh {}
impl Mesh {
    pub fn new(indices: Vec<u32>, vertices: Vec<glm::Vec4>) -> Self {
        Self { indices, vertices }
    }
}

pub struct Quad;
impl AssetLoader for Quad {
    type AssetType = Mesh;

    fn load(&mut self) -> crate::QPResult<Mesh> {
        let indices = vec![0, 1, 3, 1, 2, 3];
        let vertices = vec![
            glm::vec4(1.0, 1.0, 0.0, 1.0),
            glm::vec4(1.0, -1.0, 0.0, 1.0),
            glm::vec4(-1.0, -1.0, 0.0, 1.0),
            glm::vec4(-1.0, 1.0, 0.0, 1.0),
        ];

        Ok(Mesh { indices, vertices })
    }
}
