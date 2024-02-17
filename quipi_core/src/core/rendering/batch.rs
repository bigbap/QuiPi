use std::marker::PhantomData;

use field_offset::offset_of;

use crate::{
    opengl::{
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
        textures::{
            use_texture,
            Texture
        }
    },
    resources::{
        RShader,
        RTexture
    }
};

use super::{
    texture::from_buffer_rgba,
    vertex::Vertex, RenderInfo
};

pub struct BatchRenderer<const C: usize, M: IMesh> {
    vao: VertexArray,
    _ebo: Buffer<EBO>,
    vbo: Buffer<VBO>,

    indices_count: usize,
    white_texture: Texture,
    max_textures: u32,
    textures: Vec<u32>,
    mesh_count: usize,
    vertices: Vec<Vertex>,
    render_info: RenderInfo,

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

        let white_texture = from_buffer_rgba(1, 1, &[1, 1, 1, 1]);
        let textures = vec![white_texture.id];

        Self {
            vao,
            _ebo: ebo,
            vbo,

            indices_count: M::indices().len(),
            white_texture,
            max_textures: 16, // TODO: this is hardcoded
            textures,
            mesh_count: 0,
            vertices: vec![],
            render_info: RenderInfo::default(),

            _marker: PhantomData
        }
    }

    pub fn begin_batch(&mut self) {
        self.mesh_count = 0;
        self.textures = vec![self.white_texture.id];
        self.vertices = vec![];
    }

    pub fn flush_batch(&mut self, shader: &RShader) {
        shader.program.use_program();

        self.vbo.bind();
        self.vbo.buffer_sub_data::<Vertex>(0, self.vertices.len(), Some(&self.vertices));
        self.vbo.unbind();

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

        self.render_info.num_draw_calls += 1;
    }

    pub fn reset_info(&mut self) -> RenderInfo {
        let render_info = RenderInfo {
            num_draw_calls: self.render_info.num_draw_calls,
            ..RenderInfo::default()
        };

        self.render_info = RenderInfo::default();

        return render_info;
    }

    pub fn draw_mesh(
        &mut self,
        mesh: &M,
        shader: &RShader,
        texture: Option<&RTexture>
    ) {
        let mut texture_slot = 0;
        if let Some(texture) = texture {
            let id = texture.0.id;

            if let Some(texture) = self.textures
                .iter()
                .find(|i| **i == id)
            {
                texture_slot = *texture;
            } else {
                if self.max_textures == self.textures.len() as u32 {
                    self.flush_batch(shader);
                    self.begin_batch();
                }

                self.textures.push(id);

                texture_slot = self.textures.len() as u32 - 1;
            }
        }

        for mut vertex in mesh.vertices() {
            vertex.tex_index = texture_slot as f32;
            self.vertices.push(vertex);
        }

        self.mesh_count += 1;

        if self.mesh_count >= C {
            self.flush_batch(shader);
            self.begin_batch();
        }
    }
}

// pub struct BatchStatic<M: IMesh> {
//     pub capacity: usize,
//     pub vertex_capacity: usize,

//     pub vao: VertexArray,
//     pub ebo: Buffer<EBO>,
//     pub vbo: Buffer<VBO>,

//     _marker: PhantomData<M>
// }

// impl<M: IMesh> BatchStatic<M> {
//     pub fn new(capacity: usize, vertices: Vec<Vertex>) -> Self {
//         let stride = std::mem::size_of::<Vertex>();

//         let base_indices = M::indices();
//         let vertex_capacity = capacity * M::vertex_count();

//         if vertices.len() > vertex_capacity {
//             println!("trying to batch too many meshes");
//             panic!("tried to load a static batch with too many vertices");
//         }

//         let mut indices = Vec::<u32>::with_capacity(base_indices.len() * capacity);
//         let offset_delta = M::vertex_count();
//         for i in 0..capacity {
//             let offset = i * offset_delta;
//             for index in &base_indices {
//                 indices.push(*index as u32 + offset as u32);
//             }
//         }

//         let vao = VertexArray::new();
//         let ebo = create_ebo(&indices, &BufferUsage::StaticDraw);

//         vao.bind();
//         ebo.bind();

//         let vbo = Buffer::<VBO>::new();

//         vbo.bind();
//         vbo.buffer_data::<Vertex>(vertices.len(), Some(&vertices), &BufferUsage::StaticDraw);

//         vertex_attribute_pointer(0, 3, stride, offset_of!(Vertex => position).get_byte_offset());
//         vertex_attribute_pointer(1, 4, stride, offset_of!(Vertex => color).get_byte_offset());
//         vertex_attribute_pointer(2, 2, stride, offset_of!(Vertex => tex_coords).get_byte_offset());
//         vertex_attribute_pointer(3, 1, stride, offset_of!(Vertex => tex_index).get_byte_offset());

//         vao.unbind();
//         ebo.unbind();

//         Self {
//             capacity,
//             vertex_capacity,
//             vao,
//             ebo,
//             vbo,
//             _marker: PhantomData
//         }
//     }
// }

pub trait IMesh {
    fn vertices(&self) -> Vec<Vertex>;
    fn indices() -> Vec<i32>;
    fn vertex_count() -> usize;
}