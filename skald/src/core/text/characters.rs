use crate::gfx::pixel_store;
use ft::{face::LoadFlag, FtResult};

pub struct Character {
    texture_id: u32,
    size: glm::Vec2,
    bearing: glm::Vec2,
    advance: u32
}

pub fn load_char_textures(face: ft::Face) -> FtResult<()> {
    pixel_store::set_unpack_alignment(1);

    for c in 0..128 {
        match face.load_char(c, LoadFlag::RENDER) {
            Err(e) => {
                #[cfg(debug_assertions)]
                println!("{}", e);

                continue
            },
            _ => ()
        }
    }

    Ok(())
}
