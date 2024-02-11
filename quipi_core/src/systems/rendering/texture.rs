use std::io;
use ft::Face;

use crate::{
    systems::assets::image::Image,
    utils::to_abs_path,
    wrappers::opengl::textures::{
        Texture,
        Format,
        Target,
        ParameterName,
        ParameterValue
    },
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
    ),

    #[error("the wavefront material file doesn't have a texture path")]
    CouldntFindWavefrontTexture
}

pub fn from_buffer_rgba(
    width: i32,
    height: i32,
    buffer: &[u8]
) -> Result<Texture, TextureError> {
    let texture = Texture::new(
        width,
        height,
        Target::Texture2D
    );

    texture
        .bind()
        .add_image_data(Format::Rgba, Format::Rgba, buffer);

    Ok(texture)
}

pub fn from_wavefront_material(
    material: &tobj::Material,
) -> Result<Texture, TextureError> {

    if let Some(map_kd) = &material.diffuse_texture {
        return from_image(map_kd);
    };

    Err(TextureError::CouldntFindWavefrontTexture)
}

pub fn from_image(
    file_path: &str,
) -> Result<Texture, TextureError> {
    let file_path = &to_abs_path(file_path)?;
    let format = get_format(file_path);
    let img = Image::from_file(file_path)?;

    let texture = Texture::new(
        img.width as i32,
        img.height as i32,
        Target::Texture2D
    );

    texture
        .bind()
        .add_image_data(Format::Rgba, format, &img.flipv())
        .set_parameter(ParameterName::WrapS, ParameterValue::ClampToEdge)
        .set_parameter(ParameterName::WrapT, ParameterValue::ClampToEdge)
        .set_parameter(ParameterName::MinFilter, ParameterValue::LinearMipmapLinear)
        .set_parameter(ParameterName::MagFilter, ParameterValue::Linear);

    Ok(texture)
}

pub fn from_font(
    face: &Face,
    width: i32,
    height: i32
) -> Result<Texture, TextureError>{
    let texture = Texture::new(
        width,
        height,
        Target::Texture2D
    );

    texture
        .bind()
        .add_image_data(
            Format::Red,
            Format::Red,
            face
                .glyph()
                .bitmap()
                .buffer(),
        );

    Ok(texture)
}

fn get_format(path: &str) -> Format {
    let file_name = path
        .split('/')
        .last()
        .unwrap()
        .to_string();

    let ext = file_name.split('.').last();

    match ext {
        Some("png") => Format::Rgba,
        _ => Format::Rgb
    }
}
