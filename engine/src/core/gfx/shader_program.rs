use std::{
    fs,
    io::{self, Read},
    ffi
};

use crate::core::utils::{
    macros,
    strings
};

#[derive(Debug, thiserror::Error)]
pub enum ShaderError {
    #[error("I/O Error")]
    Io(
        #[from]
        io::Error
    ),
    
    #[error("File contains nil value")]
    FileContainsNil,
    
    #[error("There was an error compiling the shader: {}", .0)]
    CompileError(String),
    
    #[error("There was a problem linking the program")]
    LinkingError
}

#[derive(Debug)]
pub struct ShaderProgram {
    pub id: gl::types::GLuint,

    _shaders: Vec<gl::types::GLuint>
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl ShaderProgram {
    pub fn new (name: &str) -> Result<Self, ShaderError> {
        let shaders = vec![
            compile_shader(&format!("{name}.vert"), gl::VERTEX_SHADER)?,
            compile_shader(&format!("{name}.frag"), gl::FRAGMENT_SHADER)?
        ];
        let id: gl::types::GLuint = unsafe { gl::CreateProgram() };

        for shader in &shaders {
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

                let error = strings::create_empty_cstring_with_len(len as usize);
                gl::GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );

                return Err(ShaderError::LinkingError);
            }
        }

        for shader in &shaders {
            unsafe {
                gl::DetachShader(id, *shader);
            }
        }

        Ok(ShaderProgram {
            id,
            _shaders: shaders
        })
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_float_3(&self, key: &str, val: &glm::Vec3) {
        self.use_program();

        unsafe {
            gl::Uniform3f(self.get_location(key), val.x, val.y, val.z);
        }
    }

    pub fn set_float_4(&self, key: &str, val: &glm::Vec4) {
        self.use_program();

        unsafe {
            gl::Uniform4f(self.get_location(key), val.x, val.y, val.z, val.w);
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
            gl::UniformMatrix4fv(self.get_location(key), 1, gl::FALSE, glm::value_ptr(val).as_ptr());
        }
    }

    fn get_location(&self, key: &str) -> gl::types::GLint {
        unsafe {
            gl::GetUniformLocation(
                self.id,
                macros::c_str!(key).as_ptr()
            )
        }
    }
}

// helper functions

fn compile_shader(
    file_name: &str,
    kind: gl::types::GLenum
) -> Result<gl::types::GLuint, ShaderError> {
    let source = shader_to_cstring(file_name)?;
    let id = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(
            id,
            1,
            &source.as_ptr(),
            std::ptr::null()
        );
        gl::CompileShader(id);

        let mut success: gl::types::GLint = 0;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);

            let error = strings::create_empty_cstring_with_len(len as usize);
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );

            return Err(ShaderError::CompileError(file_name.to_string()));
        }
    }

    Ok(id)
}

fn shader_to_cstring(shader_path: &str) -> Result<ffi::CString, ShaderError> {
    let mut file = fs::File::open(shader_path)?;

    // allocate buffer of the same size as file
    let mut buffer: Vec<u8> = Vec::with_capacity(
        file.metadata()?.len() as usize + 1
    );
    file.read_to_end(&mut buffer)?;

    // check for null byte
    if buffer.iter().any(|i| *i == 0) {
        return Err(ShaderError::FileContainsNil);
    }

    Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
}

