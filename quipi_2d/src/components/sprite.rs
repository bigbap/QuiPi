use quipi_core::{
    opengl::buffer::BufferUsage,
    rendering::mesh::{
        ElementArrayMesh,
        ShaderLocation
    },
    systems::assets::ObjectConfig,
    Component,
    VersionedIndex
};



#[derive(Debug, Component, PartialEq)]
pub struct CSprite {
    pub shader: VersionedIndex,
    pub camera: VersionedIndex,
    pub texture: Option<VersionedIndex>,

    pub active: bool,
}

#[derive(Debug, Component, PartialEq)]
pub struct CMesh2D {
    pub mesh: Option<ElementArrayMesh>,
    pub data: ObjectConfig,
    pub usage: BufferUsage,
}

impl CMesh2D {
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
