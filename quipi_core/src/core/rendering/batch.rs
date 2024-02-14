use std::marker::PhantomData;

use field_offset::offset_of;

use crate::{
    opengl::buffer::{
        create_ebo, vertex_attribute_pointer, Buffer, BufferUsage, VertexArray, EBO, VBO
    }, VersionedIndex
};

use super::vertex::Vertex;

pub struct BatchDynamic<M: IMesh> {
    pub capacity: usize,
    pub vao: VertexArray,
    pub ebo: Buffer<EBO>,
    pub vbo: Buffer<VBO>,

    _marker: PhantomData<M>
}

impl<M: IMesh> BatchDynamic<M> {
    pub fn new(capacity: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let stride = std::mem::size_of::<Vertex>();

        let base_indices = M::indices();
        let len = base_indices.len();
        let tot_meshes = capacity / M::vertex_count();
        let mut indices = Vec::<u32>::with_capacity(len * tot_meshes);
        for i in 0..tot_meshes {
            let offset = i * len;
            for index in base_indices {
                indices.push(index as u32 + offset as u32);
            }
        }

        let vao = VertexArray::new();
        let ebo = create_ebo(&indices, &BufferUsage::StaticDraw)?;

        vao.bind();
        ebo.bind();

        let vbo = Buffer::<VBO>::new();

        vbo.bind();
        vbo.buffer_data::<Vertex>(capacity, None, &BufferUsage::DynamicDraw);

        vertex_attribute_pointer(0, 3, stride, offset_of!(Vertex => position).get_byte_offset());
        vertex_attribute_pointer(1, 4, stride, offset_of!(Vertex => color).get_byte_offset());
        vertex_attribute_pointer(2, 2, stride, offset_of!(Vertex => tex_coords).get_byte_offset());
        vertex_attribute_pointer(3, 1, stride, offset_of!(Vertex => tex_index).get_byte_offset());

        Ok(Self {
            capacity,
            vao,
            ebo,
            vbo,
            _marker: PhantomData
        })
    }

    pub fn draw(
        &mut self,
        meshes: Vec<M>
    ) {

    }
}

pub trait IMesh {
    fn vertices(&self) -> Vec<Vertex>;
    fn indices() -> Vec<i32>;
    fn vertex_count() -> usize;
}