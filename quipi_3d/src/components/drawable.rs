use quipi_core::{
    Component,
    systems::{
        assets::ObjectConfig, rendering::mesh::{
            ElementArrayMesh,
            ShaderLocation
        }
    }, wrappers::opengl::buffer::BufferUsage, VersionedIndex
};

#[derive(Debug, Component, PartialEq)]
pub struct CDrawable {
    pub shader: VersionedIndex,
    pub camera: VersionedIndex,
    pub textures: Vec<VersionedIndex>,

    pub active: bool,
}

#[derive(Debug, Component, PartialEq, Default)]
pub struct CMesh {
    pub mesh: Option<ElementArrayMesh>,
    pub data: ObjectConfig,
    pub usage: BufferUsage,
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
            data: config,
            usage
        })
    }
}
