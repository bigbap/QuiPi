use crate::wrappers::opengl::buffer::{
    VBO,
    EBO,
    VertexArray,
    Buffer,
    create_ebo,
    create_vbo,
    BufferUsage
};

#[derive(Debug, Clone, Copy)]
pub enum ShaderLocation {
    Zero,
    One,
    Two,
    Three,
    Four,
    At(usize)
}

#[derive(Debug, PartialEq)]
pub struct ElementArray {
    pub vao: VertexArray,
    pub ebo: Option<Buffer<EBO>>,
    pub vbo_list: Vec<Buffer<VBO>>,

    pub usage: BufferUsage,
}

impl ElementArray {
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
    */
    pub fn with_vbo<const S: usize, T>(
        &mut self,
        location: ShaderLocation,
        data: &[T]
    ) -> Result<&mut Self, Box<dyn std::error::Error>> {
        self.create_vbo::<S, T>(location, data.len(), data)?;

        Ok(self)
    }

    /**
    * S: number of elements to calculate the stride for. I.E. if it's a 2D texture coordinate, it should be 2
    *
    * location: location on the shader
    * buffer_length: length of the buffer. This is needed because you might want to allocate a buffer without
    * giving it data yet.
    */
    pub fn with_empty_vbo<const S: usize, T>(
        &mut self,
        location: ShaderLocation,
        buffer_length: usize,
    ) -> Result<&mut Self, Box<dyn std::error::Error>> {
        self.create_vbo::<S, T>(location, buffer_length, &[])?;

        Ok(self)
    }

    fn create_vbo<const S: usize, T>(
        &mut self,
        location: ShaderLocation,
        buffer_length: usize,
        data: &[T]
    ) -> Result<&mut Self, Box<dyn std::error::Error>> {
        if buffer_length == 0 && data.is_empty() {
            return Ok(self);
        }

        self.vao.bind();
        if let Some(ebo) = &self.ebo { ebo.bind() }
        
        let stride = std::mem::size_of::<T>() * S;
        self.vbo_list.push(create_vbo::<T>(
            Some(data),
            location.unwrap(),
            S,
            buffer_length,
            stride,
            &self.usage
        )?);

        self.vao.unbind();
        if let Some(ebo) = &self.ebo { ebo.unbind() }
        
        Ok(self)
    }
}

impl ShaderLocation {
    fn unwrap(&self) -> usize {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::At(loc) => *loc
        }
    }
}

