use crate::gfx::texture::ITexture;

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
    Rgba
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum ParameterNames {
    MinFilter,
    MagFilter,
    WrapT,
    WrapR,
    WrapS
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum ParameterValues {
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

#[derive(Debug)]
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
        buffer: &[u8]
    ) -> &Self {
        let internal_format = internal_format.unwrap();
        let format = format.unwrap();

        unsafe {
            gl::BindTexture(self.target, self.id);
            gl::TexImage2D(
                self.target,
                0,
                internal_format as i32,
                self.width,
                self.height,
                0,
                format,
                gl::UNSIGNED_BYTE,
                buffer.as_ptr() as *const std::ffi::c_void
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
        pname: ParameterNames,
        value: ParameterValues
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
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::BindTexture(self.target, 0);
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}

impl ITexture for Texture {
    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn use_texture(&self, unit: i32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit as gl::types::GLuint);
            gl::BindTexture(self.target, self.id);
        }
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
            Format::Rgba => gl::RGBA
        }
    }
}

impl ParameterNames {
    pub fn unwrap(&self) -> u32 {
        match self {
            ParameterNames::MinFilter => gl::TEXTURE_MIN_FILTER,
            ParameterNames::MagFilter => gl::TEXTURE_MAG_FILTER,
            ParameterNames::WrapT => gl::TEXTURE_WRAP_T,
            ParameterNames::WrapR => gl::TEXTURE_WRAP_R,
            ParameterNames::WrapS => gl::TEXTURE_WRAP_S,
        }
    }
}

impl ParameterValues {
    pub fn unwrap(&self) -> u32 {
        match self {
            ParameterValues::Linear => gl::LINEAR,
            ParameterValues::Nearest => gl::NEAREST,
            ParameterValues::LinearMipmapLinear => gl::LINEAR_MIPMAP_LINEAR,
            ParameterValues::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST,
            ParameterValues::NearestMipmapLinear => gl::NEAREST_MIPMAP_LINEAR,
            ParameterValues::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST,

            ParameterValues::ClampToEdge => gl::CLAMP_TO_EDGE,
            ParameterValues::ClampToBorder => gl::CLAMP_TO_BORDER,
            ParameterValues::MirroredRepeat => gl::MIRRORED_REPEAT,
            ParameterValues::Repeat => gl::REPEAT,
            ParameterValues::MirrorClampToEdge => gl::MIRROR_CLAMP_TO_EDGE,

            ParameterValues::U32(val) => *val
        }
    }
}
