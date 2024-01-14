use std::io;
use gltf::Gltf;
use image::{
    self,
    ImageBuffer,
};

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

fn new() -> Result<u32, TextureError> {
    let mut index: gl::types::GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut index);
    }

    Ok(index)
}

pub fn from_wavefront_material(
    material: &tobj::Material,
) -> Result<u32, TextureError> {
    let index = new()?;

    bind(&index);

    if let Some(map_kd) = &material.diffuse_texture {
        // found texture path
        let format = get_format(map_kd);

        add_image_from_file(
            map_kd,
            format
        )?;
    };

    if let Some(kd) = &material.diffuse {
        // found texture RGB values
        add_image_from_color(kd)?;
    };

    unbind();

    set_default_parameters(&index)?;

    Ok(index)
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
) -> Result<u32, TextureError> {
    let index = new()?;
    let format = get_format(file_path);

    set_default_parameters(&index)?;

    bind(&index);
    add_image_from_file(
        file_path,
        format
    )?;
    unbind();

    Ok(index)
}

pub fn bind(index: &u32) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, *index);
    }
}

pub fn unbind() {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }
}

fn set_default_parameters(index: &u32) -> Result<(), TextureError> {
    bind(index);

    set_parameter(gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE)?;
    set_parameter(gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE)?;
    set_parameter(gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR)?;
    set_parameter(gl::TEXTURE_MAG_FILTER, gl::LINEAR)?;

    unbind();

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

pub fn set_active_texture(unit: i32) {
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0 + unit as gl::types::GLuint);
    }
}

pub fn delete_texture(index: u32) {
    unsafe {
        gl::DeleteTextures(1, [index].as_ptr());
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
    file_path: &str,
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

fn get_format(path: &str) -> gl::types::GLenum {
    let file_name = path
        .split('/')
        .last()
        .unwrap()
        .to_string();

    let ext = file_name.split('.').last();

    match ext {
        Some("png") => gl::RGBA,
        _ => gl::RGB
    }
}
