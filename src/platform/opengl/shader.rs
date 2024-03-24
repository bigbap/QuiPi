use std::{ffi, fs, io::Read};

use super::c_str::*;
use crate::{
    prelude::{qp_core::to_abs_path, QPError},
    QPResult,
};

#[derive(Debug, PartialEq, Default)]
pub struct ShaderProgram {
    pub id: gl::types::GLuint,

    _shaders: Vec<gl::types::GLuint>,
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl ShaderProgram {
    pub fn new(name: &str) -> QPResult<Self> {
        Self::from_file(name)
    }

    pub fn from_str(vert: &str, frag: &str) -> QPResult<Self> {
        let c_vert = str_to_cstring(vert)?;
        let c_frag = str_to_cstring(frag)?;

        let shaders = vec![
            compile_shader(
                c_vert,
                gl::VERTEX_SHADER,
                QPError::CompileError(vert.to_string()),
            )?,
            compile_shader(
                c_frag,
                gl::FRAGMENT_SHADER,
                QPError::CompileError(frag.to_string()),
            )?,
        ];

        Ok(ShaderProgram {
            id: link_program(&shaders)?,
            _shaders: shaders,
        })
    }

    pub fn from_file(name: &str) -> QPResult<Self> {
        let name = &to_abs_path(&format!("assets/shaders/{}", name))?;
        let vert = shader_to_cstring(&format!("{name}.vert"))?;
        let frag = shader_to_cstring(&format!("{name}.frag"))?;

        let shaders = vec![
            compile_shader(
                vert,
                gl::VERTEX_SHADER,
                QPError::CompileError(name.to_string()),
            )?,
            compile_shader(
                frag,
                gl::FRAGMENT_SHADER,
                QPError::CompileError(name.to_string()),
            )?,
        ];

        Ok(ShaderProgram {
            id: link_program(&shaders)?,
            _shaders: shaders,
        })
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_float_2(&self, key: &str, val: (f32, f32)) {
        self.use_program();

        unsafe {
            gl::Uniform2f(self.get_location(key), val.0, val.1);
        }
    }

    pub fn set_float_3(&self, key: &str, val: (f32, f32, f32)) {
        self.use_program();

        unsafe {
            gl::Uniform3f(self.get_location(key), val.0, val.1, val.2);
        }
    }

    pub fn set_float_4(&self, key: &str, val: (f32, f32, f32, f32)) {
        self.use_program();

        unsafe {
            gl::Uniform4f(self.get_location(key), val.0, val.1, val.2, val.3);
        }
    }

    pub fn set_float(&self, key: &str, val: f32) {
        self.use_program();

        unsafe {
            gl::Uniform1f(self.get_location(key), val);
        }
    }

    pub fn set_int(&self, key: &str, val: i32) {
        self.use_program();

        unsafe {
            gl::Uniform1i(self.get_location(key), val);
        }
    }

    pub fn set_mat4(&self, key: &str, val: &glm::Mat4) {
        self.use_program();

        unsafe {
            gl::UniformMatrix4fv(
                self.get_location(key),
                1,
                gl::FALSE,
                glm::value_ptr(val).as_ptr(),
            );
        }
    }

    fn get_location(&self, key: &str) -> gl::types::GLint {
        unsafe { gl::GetUniformLocation(self.id, c_str!(key).as_ptr()) }
    }
}

// helper functions

fn link_program(shaders: &[gl::types::GLuint]) -> QPResult<gl::types::GLuint> {
    let id: gl::types::GLuint = unsafe { gl::CreateProgram() };

    for shader in shaders {
        unsafe {
            gl::AttachShader(id, *shader);
        }
    }

    unsafe {
        gl::LinkProgram(id);

        let mut success: gl::types::GLint = 0;
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);

            let error = create_empty_cstring_with_len(len as usize);
            gl::GetProgramInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );

            if cfg!(debug_assertions) {
                println!("{:?}", error);
            }

            return Err(QPError::LinkingError);
        }
    }

    for shader in shaders {
        unsafe {
            gl::DetachShader(id, *shader);
            gl::DeleteShader(*shader);
        }
    }

    Ok(id)
}

fn compile_shader(
    source: ffi::CString,
    kind: gl::types::GLenum,
    err: QPError,
) -> QPResult<gl::types::GLuint> {
    let id = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);

        let mut success: gl::types::GLint = 0;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);

            let error = create_empty_cstring_with_len(len as usize);
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );

            if cfg!(debug_assertions) {
                println!("{:?}", error);
            }

            return Err(err);
        }
    }

    Ok(id)
}

fn str_to_cstring(shader: &str) -> QPResult<ffi::CString> {
    let buffer = shader.as_bytes();

    // check for null byte
    if buffer.iter().any(|i| *i == 0) {
        return Err(QPError::FileContainsNil);
    }

    Ok(unsafe { ffi::CString::from_vec_unchecked(buffer.to_vec()) })
}

fn shader_to_cstring(shader_path: &str) -> QPResult<ffi::CString> {
    let mut file = fs::File::open(shader_path)?;

    // allocate buffer of the same size as file
    let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);
    file.read_to_end(&mut buffer)?;

    // check for null byte
    if buffer.iter().any(|i| *i == 0) {
        return Err(QPError::FileContainsNil);
    }

    Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
}
