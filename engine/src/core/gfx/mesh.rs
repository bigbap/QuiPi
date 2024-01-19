use crate::gfx::buffer::{
    self,
    EBO,
    create_ebo,
    create_vbo
};

#[derive(Debug)]
pub struct ElementArrayMesh {
    pub vao: buffer::VertexArray,
    pub ebo: buffer::Buffer<EBO>,
}

impl ElementArrayMesh {
    pub fn new(indices: &[u32]) -> Result<Self, Box<dyn std::error::Error>> {
        let ebo = create_ebo(indices)?;
        let vao = buffer::VertexArray::new(indices.len() as i32);

        Ok(Self {
            vao,
            ebo
        })
    }

    /**
    * location: location on the shader
    * size: number of elements to calculate the stride for. I.E. if it's a 2D texture coordinate, it should be 2
    */
    pub fn create_vbo_at(
        &self,
        data: &[f32],
        location: usize,
        size: usize
    ) -> Result<&Self, Box<dyn std::error::Error>> {
        self.vao.bind();
        self.ebo.bind();
        
        let stride = std::mem::size_of::<f32>() * size;
        let _vbo = create_vbo(data, location, size, stride)?;

        self.vao.unbind();
        self.ebo.unbind();
        
        Ok(self)
    }
}

