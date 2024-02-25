use std::marker::PhantomData;

use field_offset::offset_of;

use crate::{
    platform::opengl::{
        self,
        buffer::{
            create_ebo,
            vertex_attribute_pointer,
            Buffer,
            BufferUsage,
            VertexArray,
            EBO,
            VBO
        },
        draw::{
            DrawBuffer,
            DrawMode
        },
        textures::use_texture
    },
    prelude::qp_assets::{
        RShader,
        RTexture
    },
    prelude::qp_data::{
        Vertex,
        IMesh
    }
};

pub struct BatchRenderer<const C: usize, M: IMesh> {
    vao: VertexArray,
    _ebo: Buffer<EBO>,
    vbo: Buffer<VBO>,

    indices_count: usize,
    max_textures: u32,
    textures: Vec<u32>,
    mesh_count: usize,
    vertices: Vec<Vertex>,
    
    pub draw_calls: u32,

    _marker: PhantomData<M>
}

impl<const C: usize, M: IMesh> BatchRenderer<C, M> {
    pub fn new() -> Self {
        let stride = std::mem::size_of::<Vertex>();

        let base_indices = M::indices();
        let vertex_capacity = C * M::vertex_count();
        let mut indices = Vec::<u32>::with_capacity(base_indices.len() * C);
        let offset_delta = M::vertex_count();
        for i in 0..C {
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

        vao.unbind();
        ebo.unbind();

        Self {
            vao,
            _ebo: ebo,
            vbo,

            indices_count: M::indices().len(),
            max_textures: 16, // TODO: this is hardcoded
            textures: vec![],
            mesh_count: 0,
            vertices: Vec::<Vertex>::with_capacity(vertex_capacity),
            draw_calls: 0,

            _marker: PhantomData
        }
    }

    pub fn begin_batch(&mut self) {
        self.mesh_count = 0;
        self.textures.clear();
        self.vertices.clear();
    }

    pub fn flush_batch(&mut self, shader: &RShader) {
        if self.vertices.is_empty() { return; }

        shader.program.use_program();

        for i in 0..self.textures.len() {
            use_texture(self.textures[i], i as i32);
            shader.program.set_int(&format!("u_textures[{}]", i), i as i32);
        }

        self.vao.bind();
        opengl::draw::gl_draw(
            DrawBuffer::Elements,
            DrawMode::Triangles, // TODO: this is hardcoded
            (self.indices_count * self.mesh_count) as i32
        );
        self.vao.unbind();

        self.draw_calls += 1;
    }

    pub fn end_batch(&self) {
        self.vbo.bind();
        self.vbo.buffer_sub_data::<Vertex>(0, self.vertices.len(), Some(&self.vertices));
        self.vbo.unbind();
    }

    pub fn reset_info(&mut self) {
        self.draw_calls = 0;
    }

    pub fn draw_mesh(
        &mut self,
        mesh: &M,
        shader: &RShader,
        texture: Option<&RTexture>
    ) {
        let mut texture_slot = self.max_textures as usize;
        if let Some(texture) = texture {
            let id = texture.texture.id;

            if let Some((i, _)) = self.textures
                .iter()
                .enumerate()
                .find(|(_, tex)| **tex == id)
            {
                texture_slot = i;
            } else {
                if self.textures.len() as u32 >= self.max_textures {
                    self.batch_reset(shader);
                }

                self.textures.push(id);

                texture_slot = self.textures.len() - 1;
            }
        }

        for mut vertex in mesh.vertices() {
            vertex.tex_index = texture_slot as f32;
            self.vertices.push(vertex);
        }

        self.mesh_count += 1;

        if self.mesh_count >= C {
            self.batch_reset(shader);
        }
    }

    fn batch_reset(&mut self, shader: &RShader) {
        self.end_batch();
        self.flush_batch(shader);
        self.begin_batch();
    }
}
