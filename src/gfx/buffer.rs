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

pub struct BufferTypeArray;
impl BufferType for BufferTypeArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ARRAY_BUFFER;
}

pub struct BufferTypeElementArray;
impl BufferType for BufferTypeElementArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ELEMENT_ARRAY_BUFFER;
}

// pub type ArrayBuffer = Buffer<BufferTypeArray>;
// pub type ElementArrayBuffer = Buffer<BufferTypeElementArray>;

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

pub fn create_buffer<T: BufferType>(
    data: &[f32]
) -> Result<Buffer<T>, Box<dyn std::error::Error>> {
    let buffer = Buffer::<T>::new();
    buffer.bind();
    buffer.static_buffer_data(data);
    buffer.unbind();

    Ok(buffer)
}

pub fn create_ebo(
    indices: &[u32]
) -> Result<Buffer<BufferTypeElementArray>, Box<dyn std::error::Error>> {
    let index_buffer = Buffer::<BufferTypeElementArray>::new();
    index_buffer.bind();
    index_buffer.static_buffer_data(indices);
    index_buffer.unbind();

    Ok(index_buffer)
}

pub fn vertex_attribute_pointer(
    location: usize,
    size: usize,
    stride: usize,
    offset: usize
) {
    unsafe {
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            size as gl::types::GLint,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
    }
}
