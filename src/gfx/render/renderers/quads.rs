use crate::{
    assets::{AssetId, Assets},
    common::prelude::components::{CColor, CQuad, CSkip, CTexture, CTransform2D},
    gfx::render::assets::{Shader, Texture, TextureCoords},
    platform::opengl::capabilities::{gl_blending_func, gl_enable, GLBlendingFactor, GLCapability},
    plugin::Plugin,
    prelude::{
        qp_gfx::{BatchRenderer, CMatrix4, CSprite, Vertex},
        Index, Res,
        StorageId::*,
        StorageManager,
    },
    schedule::Render,
};

const BATCH_SIZE: usize = 10000;

pub struct RenderQuads {
    shader: AssetId,
    camera_id: Index,
}

impl RenderQuads {
    pub fn new(shader: AssetId, camera_id: Index) -> Self {
        Self { shader, camera_id }
    }
}

impl Plugin for RenderQuads {
    fn build(&self, app: &mut crate::prelude::App) -> crate::QPResult<()> {
        let mut renderer = BatchRenderer::<BATCH_SIZE, 4>::new(vec![0, 1, 3, 1, 2, 3]);

        let shader_id = self.shader;
        let camera_id = self.camera_id;

        app.add_system(
            Render,
            move |(storage, shaders, textures): (
                Res<StorageManager>,
                Res<Assets<Shader>>,
                Res<Assets<Texture>>,
            )| {
                let (Some(storage), Some(shaders), Some(textures)) = (storage, shaders, textures)
                else {
                    return;
                };

                let (Some(entity_storage), Some(camera_storage), Some(shader)) = (
                    storage.get(Entities),
                    storage.get(Cameras),
                    shaders.get(&shader_id),
                ) else {
                    return;
                };

                let Some(camera) = camera_storage.get::<CMatrix4>(&camera_id) else {
                    return;
                };

                let entities = match entity_storage.get_component_list::<CSprite>() {
                    Some(entities) => entities,
                    None => return,
                };

                gl_enable(GLCapability::AlphaBlending);
                gl_blending_func(
                    GLBlendingFactor::SrcAlpha,
                    GLBlendingFactor::OneMinusSrcAlpha,
                );

                renderer.reset_info();
                renderer.begin_batch();
                for row in entities.iter() {
                    let Some((entity, _)) = row else {
                        continue;
                    };
                    if entity_storage.get::<CSkip>(&entity).is_some() {
                        continue;
                    }

                    let (Some(quad), Some(transform)) = (
                        entity_storage.get::<CQuad>(&entity),
                        entity_storage.get::<CTransform2D>(&entity),
                    ) else {
                        continue;
                    };

                    let model = transform.to_matrix();
                    let mvp = camera.0 * model;

                    let texture = match entity_storage.get::<CTexture>(&entity) {
                        Some(atlas) => textures.get(&atlas.handle.id),
                        _ => None,
                    };

                    let color = entity_storage
                        .get::<CColor>(&entity)
                        .unwrap_or(&CColor(1.0, 1.0, 1.0, 1.0));
                    let color = glm::vec4(color.0, color.1, color.2, color.3);

                    renderer.draw(
                        vertices(&mvp, &texture, color, quad.positions()),
                        shader,
                        texture,
                    );
                }
                renderer.end_batch();
                renderer.flush_batch(shader);
            },
        );

        Ok(())
    }
}

pub fn vertices(
    mvp: &glm::Mat4,
    texture: &Option<&Texture>,
    color: glm::Vec4,
    positions: [glm::Vec4; 4],
) -> [Vertex; 4] {
    let pos1 = mvp * positions[0];
    let pos2 = mvp * positions[1];
    let pos3 = mvp * positions[2];
    let pos4 = mvp * positions[3];

    // println!("pos1: {:?}", pos1);
    // println!("pos2: {:?}", pos2);
    // println!("pos3: {:?}", pos3);
    // println!("pos4: {:?}", pos4);

    let coords = match texture {
        Some(tex) => tex.get_coords_at_loc((1, 1)),
        _ => TextureCoords::default(),
    };

    [
        Vertex {
            position: pos1.xyz(),
            color,
            tex_coords: coords.top_right,
            tex_index: 0.0,
        },
        Vertex {
            position: pos2.xyz(),
            color,
            tex_coords: coords.bottom_right,
            tex_index: 0.0,
        },
        Vertex {
            position: pos3.xyz(),
            color,
            tex_coords: coords.bottom_left,
            tex_index: 0.0,
        },
        Vertex {
            position: pos4.xyz(),
            color,
            tex_coords: coords.top_left,
            tex_index: 0.0,
        },
    ]
}
