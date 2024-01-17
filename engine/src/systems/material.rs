use crate::{
    Registry,
    components::material::MaterialPart,
    resources::Texture
};

pub fn s_get_texture<'a>(
    part: &MaterialPart,
    registry: &'a Registry
) -> Option<&'a Texture> {
    match part {
        MaterialPart::Texture(texture_id) => match registry.get_resource(texture_id) {
            Some(texture) => Some(texture),
            None => None
        },
        _ => None
    }
}

pub fn s_get_value(part: &MaterialPart) -> Option<(f32, f32, f32)> {
    match part {
        MaterialPart::Value(r, g, b) => Some((*r, *g, *b)),
        _ => None
    }
}
