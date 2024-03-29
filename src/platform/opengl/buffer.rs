#![allow(clippy::new_without_default)]
#![allow(clippy::upper_case_acronyms)]

use serde::{Serialize, Deserialize};
use crate::QPResult;

pub static mut BUFFER_FLAGS: u32 = gl::COLOR_BUFFER_BIT;

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum BufferUsage {
    StreamDraw,
    StreamRead,
    StreamCopy,
    #[default] StaticDraw,
    StaticRead,
    StaticCopy,
    DynamicDraw,
    DynamicRead,
    DynamicCopy
}

pub trait BufferType {
    const BUFFER_TYPE: gl::types::GLuint;
}

#[derive(Debug, PartialEq)]
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

        unsafe { gl::GenBuffers(1, &mut id) }

        Buffer { id, _marker: ::std::marker::PhantomData }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(B::BUFFER_TYPE, self.id) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(B::BUFFER_TYPE, 0) }
    }

    pub fn buffer_data<T>(
        &self,
        buffer_length: usize,
        data: Option<&[T]>,
        usage: &BufferUsage
    ) {
        unsafe {
            gl::BufferData(
                B::BUFFER_TYPE,
                (std::mem::size_of::<T>() * buffer_length) as gl::types::GLsizeiptr,
                match data {
                    Some(d) => d.as_ptr() as *const gl::types::GLvoid,
                    _ => std::ptr::null()
                },
                usage.unwrap()
            );
        }
    }

    pub fn buffer_sub_data<T>(
        &self,
        offset: isize,
        buffer_length: usize,
        data: Option<&[T]>
    ) {
        unsafe {
            gl::BufferSubData(
                B::BUFFER_TYPE,
                offset,
                (std::mem::size_of::<T>() * buffer_length) as gl::types::GLsizeiptr,
                match data {
                    Some(d) => d.as_ptr() as *const gl::types::GLvoid,
                    _ => std::ptr::null()
                }
            );
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct VBO;
impl BufferType for VBO {
    const BUFFER_TYPE: gl::types::GLuint = gl::ARRAY_BUFFER;
}

#[derive(Debug, PartialEq)]
pub struct EBO;
impl BufferType for EBO {
    const BUFFER_TYPE: gl::types::GLuint = gl::ELEMENT_ARRAY_BUFFER;
}

#[derive(Debug, PartialEq)]
pub struct VertexArray {
    id: gl::types::GLuint,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id) }
    }
}

impl VertexArray {
    pub fn new() -> Self {
        let mut id: gl::types::GLuint = 0;

        unsafe { gl::GenVertexArrays(1, &mut id) }

        Self { id }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }
}

pub fn create_vbo<T>(
    data: Option<&[T]>,
    location: usize,
    size: usize, // this is the size of the vertex attribute (i.e. vec3 will be 3)
    buffer_length: usize, // this is the number of vertex elements in the buffer
    stride: usize,
    usage: &BufferUsage
) -> QPResult<Buffer<VBO>> {
    let buffer = Buffer::<VBO>::new();
    buffer.bind();
    buffer.buffer_data::<T>(buffer_length, data, usage);

    vertex_attribute_pointer(location, size, stride, 0);

    buffer.unbind();

    Ok(buffer)
}

pub fn vertex_attribute_pointer(
    location: usize,
    size: usize, // this is the size of the vertex attribute (i.e. vec3 will be 3)
    stride: usize,
    offset: usize
) {
    unsafe {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            size as gl::types::GLint,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        )
    }
}

pub fn create_ebo(
    indices: &[u32],
    usage: &BufferUsage
) -> Buffer<EBO> {
    let index_buffer = Buffer::<EBO>::new();
    index_buffer.bind();
    index_buffer.buffer_data(indices.len(), Some(indices), usage);
    index_buffer.unbind();

    index_buffer
}

pub fn clear_buffers(clr: (f32, f32, f32, f32)) {
    unsafe {
        gl::ClearColor(clr.0, clr.1, clr.2, clr.3);
        gl::Clear(BUFFER_FLAGS);
    }
}

// private helpers

impl BufferUsage {
    fn unwrap(&self) -> gl::types::GLenum {
        match self {
            BufferUsage::StreamDraw => gl::STREAM_DRAW,
            BufferUsage::StreamRead => gl::STREAM_READ,
            BufferUsage::StreamCopy => gl::STREAM_COPY,
            BufferUsage::StaticDraw => gl::STATIC_DRAW,
            BufferUsage::StaticRead => gl::STATIC_READ,
            BufferUsage::StaticCopy => gl::STATIC_COPY,
            BufferUsage::DynamicDraw => gl::DYNAMIC_DRAW,
            BufferUsage::DynamicRead => gl::DYNAMIC_READ,
            BufferUsage::DynamicCopy => gl::DYNAMIC_COPY
        }
    }
}
