pub mod texture {
    use crate::{
        QPResult,
        prelude::QPError,
        prelude::qp_core::{
            QPImage,
            to_abs_path,
        },
        platform::opengl::textures::{
            Texture,
            Format,
            Target,
        },
    };

    pub fn from_buffer_rgba(
        width: i32,
        height: i32,
        buffer: &[u8]
    ) -> Texture {
        let texture = Texture::new(
            width,
            height,
            Target::Texture2D
        );

        texture
            .bind()
            .add_image_data(Format::Rgba, Format::Rgba, buffer);

        texture
    }

    pub fn from_wavefront_material(
        material: &tobj::Material,
    ) -> QPResult<Texture> {

        if let Some(map_kd) = &material.diffuse_texture {
            return from_image(map_kd);
        };

        Err(QPError::CouldntFindWavefrontTexture)
    }

    pub fn from_image(
        file_path: &str,
    ) -> QPResult<Texture> {
        let file_path = &to_abs_path(file_path)?;
        let format = get_format(file_path);
        let img = QPImage::from_file(file_path)?;

        let texture = Texture::new(
            img.width as i32,
            img.height as i32,
            Target::Texture2D
        );

        texture
            .bind()
            .add_image_data(Format::Rgba, format, &img.flipv());
            

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
}
