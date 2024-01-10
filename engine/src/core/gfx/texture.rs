use std::io;
use gltf::Gltf;
use image::{
    self,
    ImageBuffer,
};

// const ASSET_PATH: &str = "assets/";

#[derive(Debug, thiserror::Error)]
pub enum TextureError {
    #[error("There was a problem loading the image")]
    ImageError(
        #[from]
        image::ImageError
    ),

    #[error("There was a problem adding the texture image")]
    FailedAddingTextureImage,
    
    #[error("There was a problem adding a parameter to the texture")]
    FailedAddingParameter,

    #[error("there was a problem reading from file")]
    ProblemReadingFile(
        #[from]
        #[source]
        io::Error
    ),

    #[error("there was a problem loading wavefront file")]
    ProblemLoadingWavefrontObj(
        #[from]
        #[source]
        tobj::LoadError
    )
}

pub struct Texture {
    pub id: u32
}

impl Drop for Texture {
    fn drop(&mut self) {
        todo!()
    }
}

impl Texture {
    fn new() -> Result<Self, TextureError> {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }

        Ok(Self { id })
    }
    
    pub fn from_wavefront_material(
        material: &tobj::Material,
    ) -> Result<Self, TextureError> {
        let texture = Self::new()?;

        Self::bind(&texture);

        if let Some(map_kd) = &material.diffuse_texture {
            // found texture path
            let (file_name, format) = parse_file_name(map_kd);

            add_image_from_file(
                file_name,
                format
            )?;
        };

        if let Some(kd) = &material.diffuse {
            // found texture RGB values
            add_image_from_color(kd)?;
        };

        Self::unbind();

        Self::set_default_parameters(&texture)?;

        Ok(texture)
    }

    pub fn from_gltf(file_path: &str) {
        let gltf = Gltf::open(file_path).unwrap();
        for scene in gltf.scenes() {
            for node in scene.nodes() {
                println!(
                    "Node #{} has {} children",
                    node.index(),
                    node.children().count(),
                );
            }
        }

        todo!()
    }

    pub fn from_image(
        file_path: &str
    ) -> Result<Self, TextureError> {
        let texture = Self::new()?;
        let (file_name, format) = parse_file_name(file_path);

        Self::bind(&texture);
        add_image_from_file(
            file_name,
            format
        )?;
        Self::unbind();

        Self::set_default_parameters(&texture)?;

        Ok(texture)
    }

    pub fn bind(texture: &Self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn set_default_parameters(texture: &Self) -> Result<(), TextureError> {
        Self::bind(texture);

        Self::set_parameter(gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE)?;
        Self::set_parameter(gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE)?;
        Self::set_parameter(gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR)?;
        Self::set_parameter(gl::TEXTURE_MAG_FILTER, gl::LINEAR)?;

        Self::unbind();

        Ok(())
    }

    pub fn set_parameter(
        name: gl::types::GLenum,
        param: gl::types::GLuint
    ) -> Result<(), TextureError> {
        unsafe {
            gl::TexParameteri(
                gl::TEXTURE_2D,
                name,
                param.try_into().map_err(|_| TextureError::FailedAddingParameter)?
            );
        }

        Ok(())
    }

    pub fn set_active_texture(unit: gl::types::GLuint) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
        }
    }
}

fn add_image_from_color(color: &[f32; 3]) -> Result<(), TextureError> {
    let img_buf = ImageBuffer::from_pixel(1, 1, image::Rgb([
        (color[0] * 256.0) as u8,
        (color[1] * 256.0) as u8,
        (color[2] * 256.0) as u8
    ]));
    let width: i32 = img_buf.width().try_into().map_err(|_| TextureError::FailedAddingTextureImage)?;
    let height: i32 = img_buf.height().try_into().map_err(|_| TextureError::FailedAddingTextureImage)?;

    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB8.try_into().map_err(|_| TextureError::FailedAddingTextureImage)?,
            width,
            height,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            img_buf.as_ptr() as *const std::ffi::c_void
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    Ok(())
}

fn add_image_from_file(
    file_path: String,
    format: gl::types::GLenum
) -> Result<(), TextureError> {
    let img = image::open(file_path)?.flipv();
    let width = img.width().try_into().map_err(|_| TextureError::FailedAddingTextureImage)?;
    let height = img.height().try_into().map_err(|_| TextureError::FailedAddingTextureImage)?;

    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB8.try_into().map_err(|_| TextureError::FailedAddingTextureImage)?,
            width,
            height,
            0,
            format,
            gl::UNSIGNED_BYTE,
            img.as_bytes().as_ptr() as *const std::ffi::c_void
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    Ok(())
}

fn parse_file_name(path: &str) -> (String, gl::types::GLenum) {
    let file_name = path
        .split('/')
        .last()
        .unwrap()
        .to_string();

    let ext = file_name.split('.').last();

    let format = match ext {
        Some("png") => gl::RGBA,
        _ => gl::RGB
    };

    (file_name, format)
}