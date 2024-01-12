#![allow(clippy::new_without_default)]

pub trait BufferType {
    const BUFFER_TYPE: gl::types::GLuint;
}

pub struct Buffer<B> where B: BufferType {
    id: gl::types::GLuint,
    _marker: ::std::marker::PhantomData<B>
}

impl<B> Drop for Buffer<B> where B: BufferType {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

impl<B> Buffer<B> where B: BufferType {
    pub fn new() -> Buffer<B> {
        let mut id: gl::types::GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        Buffer {
            id,
            _marker: ::std::marker::PhantomData
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(B::BUFFER_TYPE, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(B::BUFFER_TYPE, 0);
        }
    }

    pub fn static_buffer_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                B::BUFFER_TYPE,
                std::mem::size_of_val(data) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );
        }
    }
}

pub struct VBO;
impl BufferType for VBO {
    const BUFFER_TYPE: gl::types::GLuint = gl::ARRAY_BUFFER;
}

pub struct EBO;
impl BufferType for EBO {
    const BUFFER_TYPE: gl::types::GLuint = gl::ELEMENT_ARRAY_BUFFER;
}

#[derive(Debug)]
pub struct VertexArray {
    id: gl::types::GLuint,
    count: i32
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

impl VertexArray {
    pub fn new(count: i32) -> Self {
        let mut id: gl::types::GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        Self {
            id,
            count
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn count(&self) -> i32 { self.count }
}

pub fn create_vbo(
    data: &[f32],
    location: usize,
    size: usize,
    stride: usize
) -> Result<Buffer<VBO>, Box<dyn std::error::Error>> {
    let buffer = Buffer::<VBO>::new();
    buffer.bind();
    buffer.static_buffer_data(data);

    unsafe {
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            size as gl::types::GLint,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            std::ptr::null()
        );
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
    }

    buffer.unbind();

    Ok(buffer)
}

pub fn create_ebo(
    indices: &[u32]
) -> Result<Buffer<EBO>, Box<dyn std::error::Error>> {
    let index_buffer = Buffer::<EBO>::new();
    index_buffer.bind();
    index_buffer.static_buffer_data(indices);
    index_buffer.unbind();

    Ok(index_buffer)
}

pub fn clear_buffer(clr: Option<(f32, f32, f32, f32)>) {
    unsafe {
        let clr = clr.unwrap_or((0.02, 0.02, 0.02, 1.0));

        gl::ClearColor(clr.0, clr.1, clr.2, clr.3);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}
