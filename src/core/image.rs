use image::{
    self,
    ImageBuffer,
    DynamicImage,
    ImageError,
};

pub struct QPImage {
    pub width: u32,
    pub height: u32,
    img: DynamicImage
}

impl QPImage {
    pub fn from_file(path: &str) -> Result<Self, ImageError> {
        let img = image::open(path)?;
        let width = img.width();
        let height = img.height();
        
        Ok(Self {
            width,
            height,
            img
        })
    }

    pub fn from_pixel_3(
        color: &[f32; 3]
    ) -> Result<Self, ImageError> {
        let img_buf = ImageBuffer::from_pixel(1, 1, image::Rgb([
            (color[0] * 256.0) as u8,
            (color[1] * 256.0) as u8,
            (color[2] * 256.0) as u8
        ]));

        Ok(Self {
            width: 1,
            height: 1,
            img: DynamicImage::ImageRgb8(img_buf)
        })
    }

    pub fn from_rgb(
        width: u32,
        height: u32,
        data: &[u8]
    ) -> Result<Self, ImageError> {
        assert!(data.len() % 3 == 0);

        let img = DynamicImage::new_rgb8(width, height);
        
        Ok(Self {
            width,
            height,
            img
        })
    }

    pub fn data(&self) -> Vec<u8> {
        self.img.as_bytes().to_vec()
    }

    pub fn flipv(&self) -> Vec<u8> {
        self.img.flipv().as_bytes().to_vec()
    }

    pub fn fliph(&self) -> Vec<u8> {
        self.img.fliph().as_bytes().to_vec()
    }
}

