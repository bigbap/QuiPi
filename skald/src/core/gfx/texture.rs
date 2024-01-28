use std::{io, fmt};
use ft::Face;

use crate::utils::to_abs_path;
use super::{
    opengl::textures::{
        Texture,
        Format,
        Target,
        ParameterNames,
        ParameterValues
    },
    image::Image
};

pub use super::opengl::textures::use_texture_unit as gl_use_texture_unit;

pub trait ITexture: fmt::Debug {
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn use_texture(&self, unit: i32);
}

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


pub fn from_wavefront_material(
    material: &tobj::Material,
) -> Result<Box<dyn ITexture>, TextureError> {

    if let Some(map_kd) = &material.diffuse_texture {
        return from_image(map_kd);
    };

    Err(TextureError::CouldntFindWavefrontTexture)
}

pub fn from_image(
    file_path: &str
) -> Result<Box<dyn ITexture>, TextureError> {
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
        .set_parameter(ParameterNames::WrapS, ParameterValues::ClampToEdge)
        .set_parameter(ParameterNames::WrapT, ParameterValues::ClampToEdge)
        .set_parameter(ParameterNames::MinFilter, ParameterValues::LinearMipmapLinear)
        .set_parameter(ParameterNames::MagFilter, ParameterValues::Linear);

    Ok(Box::new(texture))
}

pub fn from_font(
    face: &Face,
    width: i32,
    height: i32
) -> Result<Box<dyn ITexture>, TextureError>{
    let texture = Texture::new(
        width,
        height,
        Target::Texture2D
    );

    texture
        .bind()
        .add_image_data(Format::Red, Format::Red, face.glyph().bitmap().buffer());

    Ok(Box::new(texture))
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
