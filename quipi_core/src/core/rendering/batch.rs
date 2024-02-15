use std::marker::PhantomData;

use field_offset::offset_of;

use crate::opengl::buffer::{
    create_ebo,
    vertex_attribute_pointer,
    Buffer,
    BufferUsage,
    VertexArray,
    EBO,
    VBO
};

use super::vertex::Vertex;

pub struct BatchDynamic<M: IMesh> {
    pub capacity: usize,
    pub vertex_capacity: usize,

    pub vao: VertexArray,
    pub ebo: Buffer<EBO>,
    pub vbo: Buffer<VBO>,

    _marker: PhantomData<M>
}

impl<M: IMesh> BatchDynamic<M> {
    pub fn new(capacity: usize) -> Self {
        let stride = std::mem::size_of::<Vertex>();

        let base_indices = M::indices();
        let vertex_capacity = capacity * M::vertex_count();
        let mut indices = Vec::<u32>::with_capacity(base_indices.len() * capacity);
        let offset_delta = M::vertex_count();
        for i in 0..capacity {
            let offset = i * offset_delta;
            for index in &base_indices {
                indices.push(*index as u32 + offset as u32);
            }
        }

        let vao = VertexArray::new();
        let ebo = create_ebo(&indices, &BufferUsage::StaticDraw);

        vao.bind();
        ebo.bind();

        let vbo = Buffer::<VBO>::new();

        vbo.bind();
        vbo.buffer_data::<Vertex>(vertex_capacity, None, &BufferUsage::DynamicDraw);

        vertex_attribute_pointer(0, 3, stride, offset_of!(Vertex => position).get_byte_offset());
        vertex_attribute_pointer(1, 4, stride, offset_of!(Vertex => color).get_byte_offset());
        vertex_attribute_pointer(2, 2, stride, offset_of!(Vertex => tex_coords).get_byte_offset());
        vertex_attribute_pointer(3, 1, stride, offset_of!(Vertex => tex_index).get_byte_offset());

        Self {
            capacity,
            vertex_capacity,
            vao,
            ebo,
            vbo,
            _marker: PhantomData
        }
    }

    pub fn update(
        &mut self,
        vertices: Vec<Vertex>
    ) {
        if vertices.len() > self.vertex_capacity {
            println!("trying to batch too many meshes");
            return;
        }

        self.vbo.bind();
        self.vbo.buffer_sub_data::<Vertex>(0, vertices.len(), Some(&vertices));
        self.vbo.unbind();
    }
}

pub struct BatchStatic<M: IMesh> {
    pub capacity: usize,
    pub vertex_capacity: usize,

    pub vao: VertexArray,
    pub ebo: Buffer<EBO>,
    pub vbo: Buffer<VBO>,

    _marker: PhantomData<M>
}

impl<M: IMesh> BatchStatic<M> {
    pub fn new(capacity: usize, vertices: Vec<Vertex>) -> Self {
        let stride = std::mem::size_of::<Vertex>();

        let base_indices = M::indices();
        let vertex_capacity = capacity * M::vertex_count();

        if vertices.len() > vertex_capacity {
            println!("trying to batch too many meshes");
            panic!("tried to load a static batch with too many vertices");
        }

        let mut indices = Vec::<u32>::with_capacity(base_indices.len() * capacity);
        let offset_delta = M::vertex_count();
        for i in 0..capacity {
            let offset = i * offset_delta;
            for index in &base_indices {
                indices.push(*index as u32 + offset as u32);
            }
        }

        let vao = VertexArray::new();
        let ebo = create_ebo(&indices, &BufferUsage::StaticDraw);

        vao.bind();
        ebo.bind();

        let vbo = Buffer::<VBO>::new();

        vbo.bind();
        vbo.buffer_data::<Vertex>(vertices.len(), Some(&vertices), &BufferUsage::StaticDraw);

        vertex_attribute_pointer(0, 3, stride, offset_of!(Vertex => position).get_byte_offset());
        vertex_attribute_pointer(1, 4, stride, offset_of!(Vertex => color).get_byte_offset());
        vertex_attribute_pointer(2, 2, stride, offset_of!(Vertex => tex_coords).get_byte_offset());
        vertex_attribute_pointer(3, 1, stride, offset_of!(Vertex => tex_index).get_byte_offset());

        Self {
            capacity,
            vertex_capacity,
            vao,
            ebo,
            vbo,
            _marker: PhantomData
        }
    }
}

pub trait IMesh {
    fn vertices(&self) -> Vec<Vertex>;
    fn indices() -> Vec<i32>;
    fn vertex_count() -> usize;
}