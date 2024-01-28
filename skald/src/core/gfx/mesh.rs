use super::opengl::buffer::{
    VBO,
    EBO,
    VertexArray,
    Buffer,
    create_ebo,
    create_vbo,
};

pub use super::opengl::buffer::BufferUsage;

#[derive(Debug)]
pub struct ElementArrayMesh {
    pub vao: VertexArray,
    pub ebo: Option<Buffer<EBO>>,
    pub vbo_list: Vec<Buffer<VBO>>,

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
            vbo_list: vec![],
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
    *
    * location: location on the shader
    * buffer_length: length of the buffer. This is needed because you might want to allocate a buffer without
    * giving it data yet.
    */
    pub fn create_vbo<const S: usize, T>(
        &mut self,
        location: usize,
        buffer_length: usize,
        data: Option<&[T]>
    ) -> Result<&mut Self, Box<dyn std::error::Error>> {
        self.vao.bind();
        if let Some(ebo) = &self.ebo { ebo.bind() }
        
        let stride = std::mem::size_of::<T>() * S;
        self.vbo_list.push(create_vbo::<T>(
            data,
            location,
            S,
            buffer_length,
            stride,
            &self.usage
        )?);

        self.vao.unbind();
        if let Some(ebo) = &self.ebo { ebo.unbind() }
        
        Ok(self)
    }

    pub fn create_vbo_2_f32(
        &mut self,
        location: usize,
        buffer_length: usize,
        data: Option<&[f32]>
    ) -> Result<&mut Self, Box<dyn std::error::Error>> {
        self.create_vbo::<2, f32>(location, buffer_length, data)
    }

    pub fn create_vbo_3_f32(
        &mut self,
        location: usize,
        buffer_length: usize,
        data: Option<&[f32]>
    ) -> Result<&mut Self, Box<dyn std::error::Error>> {
        self.create_vbo::<3, f32>(location, buffer_length, data)
    }

    pub fn create_vbo_4_f32(
        &mut self,
        location: usize,
        buffer_length: usize,
        data: Option<&[f32]>
    ) -> Result<&mut Self, Box<dyn std::error::Error>> {
        self.create_vbo::<4, f32>(location, buffer_length, data)
    }
}

