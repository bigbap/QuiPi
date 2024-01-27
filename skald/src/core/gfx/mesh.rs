use std::collections::HashMap;

use super::opengl::buffer::{
    VBO,
    EBO,
    VertexArray,
    Buffer,
    create_ebo,
    create_vbo,
};

pub use super::opengl::buffer::BufferUsage;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum VboKind {
    Vertex,
    Color,
    UVCoords,
    Normals,
}

#[derive(Debug)]
pub struct ElementArrayMesh {
    pub vao: VertexArray,
    pub ebo: Option<Buffer<EBO>>,
    pub vbo_map: HashMap<VboKind, Buffer<VBO>>,

    pub usage: BufferUsage,
}

impl ElementArrayMesh {
    pub fn new(
        length: usize,
        usage: BufferUsage
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let vao = VertexArray::new(length as i32);

        Ok(Self {
            vao,
            ebo: None,
            vbo_map: HashMap::new(),
            usage
        })
    }

    pub fn with_ebo(
        &mut self,
        indices: &[u32],
    ) -> Result<&mut Self, Box<dyn std::error::Error>> {
        self.ebo = Some(
            create_ebo(indices, &self.usage)?
        );

        Ok(self)
    }

    /**
    * S: number of elements to calculate the stride for. I.E. if it's a 2D texture coordinate, it should be 2
    * L: location on the shader
    *
    * buffer_length: length of the buffer. This is needed because you might want to allocate a buffer without
    * giving it data yet.
    */
    pub fn create_vbo<const L: usize, const S: usize, T>(
        &mut self,
        kind: VboKind,
        buffer_length: usize,
        data: Option<&[T]>
    ) -> Result<&mut Self, Box<dyn std::error::Error>> {
        self.vao.bind();
        if let Some(ebo) = &self.ebo { ebo.bind() }
        
        let stride = std::mem::size_of::<T>() * S;
        let vbo = create_vbo::<T>(
            data,
            L,
            S,
            buffer_length,
            stride,
            &self.usage
        )?;

        self.vao.unbind();
        if let Some(ebo) = &self.ebo { ebo.unbind() }

        self.vbo_map.insert(kind, vbo);
        
        Ok(self)
    }

    pub fn create_vbo_2_f32<const L: usize>(
        &mut self,
        kind: VboKind,
        buffer_length: usize,
        data: Option<&[f32]>
    ) -> Result<&mut Self, Box<dyn std::error::Error>> {
        self.create_vbo::<2, L, f32>(kind, buffer_length, data)
    }

    pub fn create_vbo_3_f32<const L: usize>(
        &mut self,
        kind: VboKind,
        buffer_length: usize,
        data: Option<&[f32]>
    ) -> Result<&mut Self, Box<dyn std::error::Error>> {
        self.create_vbo::<3, L, f32>(kind, buffer_length, data)
    }

    pub fn create_vbo_4_f32<const L: usize>(
        &mut self,
        kind: VboKind,
        buffer_length: usize,
        data: Option<&[f32]>
    ) -> Result<&mut Self, Box<dyn std::error::Error>> {
        self.create_vbo::<4, L, f32>(kind, buffer_length, data)
    }
}

