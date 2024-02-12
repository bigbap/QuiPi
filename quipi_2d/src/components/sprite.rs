use quipi_core::{
    opengl::{buffer::BufferUsage, draw::DrawMode},
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
}

#[derive(Debug, Component, PartialEq)]
pub struct CMesh2D {
    pub mesh: ElementArrayMesh,
    pub data: ObjectConfig,
    pub draw_mode: DrawMode,
    pub usage: BufferUsage,
    pub should_draw: bool
}

impl CMesh2D {
    pub fn new(
        config: ObjectConfig,
        draw_mode: DrawMode,
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
            mesh,
            data: config,
            draw_mode,
            usage,
            should_draw: true
        })
    }
}
