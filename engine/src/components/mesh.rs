use crate::Component;
use crate::core::gfx::{
    object_loader::ObjectConfig,    
    buffer::{
        self,
        create_ebo,
        create_vbo,
        VBO
    },
};

pub const SHADER_POSITION_LOCATION: usize = 0;
pub const SHADER_COLOR_LOCATION: usize = 1;
pub const SHADER_TEXCOORD_LOCATION: usize = 2;
pub const SHADER_NORMALS_LOCATION: usize = 3;

pub const SIZE_OF_F32_3: usize = std::mem::size_of::<f32>() * 3;
pub const SIZE_OF_F32_2: usize = std::mem::size_of::<f32>() * 2;

#[derive(Component, Debug)]
pub struct MeshComponent {
    vao: buffer::VertexArray,
}

impl MeshComponent {
    pub fn new(
        config: &ObjectConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let vao = Self::load_vao(config)?;

        Ok(Self {
            vao
        })
    }

    fn load_vao(
        config: &ObjectConfig
    ) -> Result<buffer::VertexArray, Box<dyn std::error::Error>> {
        let ebo = create_ebo(&config.indices)?;
        let vao = buffer::VertexArray::new(config.indices.len() as i32);
    
        vao.bind();
        ebo.bind();

        #[cfg(debug_assertions)]
        {
            println!("indices: {:?}", config.indices);
            println!("position: {:?}", config.positions);
        }

        // if the vbo isn't assigned to a variable, opengl crashes with STATUS_ACCESS_VIOLATION
        let mut _vbo_list: Vec<buffer::Buffer<VBO>> = vec![];
        _vbo_list.push(create_vbo(&config.positions, SHADER_POSITION_LOCATION, 3, SIZE_OF_F32_3)?);
        
        if !config.normals.is_empty() {
            #[cfg(debug_assertions)]
            {
                println!("normals: {:?}", config.normals);
            }
            _vbo_list.push(create_vbo(&config.normals, SHADER_NORMALS_LOCATION, 3, SIZE_OF_F32_3)?);
        }
        if !config.colors.is_empty() {
            #[cfg(debug_assertions)]
            {
                println!("colors: {:?}", config.colors);
            }
            _vbo_list.push(create_vbo(&config.colors, SHADER_COLOR_LOCATION, 3, SIZE_OF_F32_3)?);
        }
        if !config.texture_coords.is_empty() {
            #[cfg(debug_assertions)]
            {
                println!("texture coords: {:?}", config.texture_coords);
            }
            _vbo_list.push(create_vbo(&config.texture_coords, SHADER_TEXCOORD_LOCATION, 2, SIZE_OF_F32_2)?);
        }

        vao.unbind();
        ebo.unbind();

        Ok(vao)
    }

    pub fn vao(&self) -> &buffer::VertexArray {
        &self.vao
    }
}
