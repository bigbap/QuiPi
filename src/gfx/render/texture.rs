pub mod texture {
    use crate::{
        platform::opengl::textures::{Format, GlTexture, Target},
        prelude::qp_core::{to_abs_path, QPImage},
        prelude::QPError,
        QPResult,
    };

    pub fn from_buffer(format: Format, width: i32, height: i32, buffer: &[u8]) -> GlTexture {
        let texture = GlTexture::new(width, height, Target::Texture2D);

        texture.bind().add_image_data(format, format, buffer);

        texture
    }

    pub fn from_wavefront_material(material: &tobj::Material) -> QPResult<GlTexture> {
        if let Some(map_kd) = &material.diffuse_texture {
            return from_image(map_kd);
        };

        Err(QPError::CouldntFindWavefrontTexture)
    }

    pub fn from_image(file_path: &str) -> QPResult<GlTexture> {
        let file_path = &to_abs_path(file_path)?;
        let format = get_format(file_path);
        let img = QPImage::from_file(file_path)?;

        let texture = GlTexture::new(img.width as i32, img.height as i32, Target::Texture2D);

        texture
            .bind()
            .add_image_data(Format::Rgba, format, &img.flipv());

        Ok(texture)
    }

    fn get_format(path: &str) -> Format {
        let file_name = path.split('/').last().unwrap().to_string();

        let ext = file_name.split('.').last();

        match ext {
            Some("png") => Format::Rgba,
            _ => Format::Rgb,
        }
    }
}
