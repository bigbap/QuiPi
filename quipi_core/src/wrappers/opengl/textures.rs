/**
* Public API
*/

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Target {
    Texture1D,
    Texture2D,
    Texture3D
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Rgb,
    Rgba,
    Red,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum ParameterName {
    MinFilter,
    MagFilter,
    WrapT,
    WrapR,
    WrapS
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum ParameterValue {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,

    ClampToEdge,
    ClampToBorder,
    MirroredRepeat,
    Repeat,
    MirrorClampToEdge,
    
    U32(u32)
}

#[derive(Debug, PartialEq)]
pub struct Texture {
    id: u32,
    target: gl::types::GLenum,

    pub width: i32,
    pub height: i32,
}

impl Texture {
    pub fn new(
        width: i32,
        height: i32,
        target: Target
    ) -> Self {
        let mut id: gl::types::GLuint = 0;
        unsafe { gl::GenTextures(1, &mut id) }

        Self {
            id,
            width,
            height,
            target: target.unwrap()
        }
    }

    pub fn add_image_data(
        &self,
        internal_format: Format,
        format: Format,
        buffer: &[u8],
    ) -> &Self {
        let internal_format = internal_format.unwrap();
        let format = format.unwrap();

        unsafe {
            gl::TexImage2D(
                self.target,
                0,
                internal_format as i32,
                self.width,
                self.height,
                0,
                format,
                gl::UNSIGNED_BYTE,
                buffer.as_ptr() as *const gl::types::GLvoid
            );

            gl::GenerateMipmap(self.target);
            gl::BindTexture(self.target, 0);
        }

        self
    }

    pub fn sub_image_data(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: Format,
        buffer: &[u8],
    ) -> &Self {
        let format = format.unwrap();

        unsafe {
            gl::TexSubImage2D(
                self.target,
                0,
                x,
                y,
                width,
                height,
                format,
                gl::UNSIGNED_BYTE,
                buffer.as_ptr() as *const gl::types::GLvoid
            );

            gl::GenerateMipmap(self.target);
            gl::BindTexture(self.target, 0);
        }

        self
    }

    pub fn bind(&self) -> &Self {
        unsafe { gl::BindTexture(self.target, self.id) }

        self
    }

    pub fn set_parameter(
        &self,
        pname: ParameterName,
        value: ParameterValue
    ) -> &Self {
        unsafe {
            gl::TexParameteri(
                self.target,
                pname.unwrap(),
                value.unwrap() as i32
            );
        }

        self
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn use_texture(&self, unit: i32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit as gl::types::GLuint);
            gl::BindTexture(self.target, self.id);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::BindTexture(self.target, 0);
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}

pub fn gl_use_texture_unit(unit: i32) {
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0 + unit as gl::types::GLuint);
    }
}

/**
* Private
*/

impl Target {
    fn unwrap(&self) -> gl::types::GLenum {
        match self {
            Target::Texture1D => gl::TEXTURE_1D,
            Target::Texture2D => gl::TEXTURE_2D,
            Target::Texture3D => gl::TEXTURE_3D
        }
    }
}

impl Format {
    fn unwrap(&self) -> gl::types::GLenum {
        match self {
            Format::Rgb => gl::RGB,
            Format::Rgba => gl::RGBA,
            Format::Red => gl::RED
        }
    }
}

impl ParameterName {
    pub fn unwrap(&self) -> u32 {
        match self {
            ParameterName::MinFilter => gl::TEXTURE_MIN_FILTER,
            ParameterName::MagFilter => gl::TEXTURE_MAG_FILTER,
            ParameterName::WrapT => gl::TEXTURE_WRAP_T,
            ParameterName::WrapR => gl::TEXTURE_WRAP_R,
            ParameterName::WrapS => gl::TEXTURE_WRAP_S,
        }
    }
}

impl ParameterValue {
    pub fn unwrap(&self) -> u32 {
        match self {
            ParameterValue::Linear => gl::LINEAR,
            ParameterValue::Nearest => gl::NEAREST,
            ParameterValue::LinearMipmapLinear => gl::LINEAR_MIPMAP_LINEAR,
            ParameterValue::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST,
            ParameterValue::NearestMipmapLinear => gl::NEAREST_MIPMAP_LINEAR,
            ParameterValue::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST,

            ParameterValue::ClampToEdge => gl::CLAMP_TO_EDGE,
            ParameterValue::ClampToBorder => gl::CLAMP_TO_BORDER,
            ParameterValue::MirroredRepeat => gl::MIRRORED_REPEAT,
            ParameterValue::Repeat => gl::REPEAT,
            ParameterValue::MirrorClampToEdge => gl::MIRROR_CLAMP_TO_EDGE,

            ParameterValue::U32(val) => *val
        }
    }
}
