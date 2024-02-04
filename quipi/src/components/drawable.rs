use crate::{
    Component,
    systems::{
        rendering::mesh::{
            ElementArrayMesh,
            ShaderLocation
        },
        assets::ObjectConfig
    },
    VersionedIndex,
    wrappers::opengl::buffer::BufferUsage
};

#[derive(Debug, Component)]
pub struct CMesh {
    pub mesh: Option<ElementArrayMesh>,
    pub data: ObjectConfig,
}

impl CMesh {
    pub fn new(
        config: ObjectConfig,
        usage: BufferUsage
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut mesh = ElementArrayMesh::new(
            config.indices.len(),
            usage
        )?;

        // TODO:
        mesh
            .with_ebo(&config.indices)?
            .with_vbo::<3, f32>(ShaderLocation::Zero, &config.points)?
            .with_vbo::<4, f32>(ShaderLocation::One, &config.colors)?;

        Ok(Self {
            mesh: Some(mesh),
            data: config
        })
    }
}

// shader_tag is used during serialization
#[derive(Debug, Component)]
pub struct CShader {
    pub shader: VersionedIndex,
}

