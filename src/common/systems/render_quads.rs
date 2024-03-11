use crate::{
    common::{
        assets::{ShaderAsset, TextureAsset, TextureCoords},
        bundles::CSprite,
        prelude::components::{CQuad, CSkip, CTextureAtlas, CTransform2D},
        resources::{AssetId, Camera2D, CameraId},
    },
    platform::opengl::capabilities::{gl_blending_func, gl_enable, GLBlendingFactor, GLCapability},
    prelude::{
        qp_gfx::{BatchRenderer, Vertex},
        QPError, System,
    },
    registry::GlobalRegistry,
};

const BATCH_SIZE: usize = 10000;

pub fn render_quads(shader_id: AssetId, camera_id: CameraId) -> impl System {
    debug_assert!(shader_id.validate::<ShaderAsset>());

    let mut renderer = BatchRenderer::<BATCH_SIZE, 4>::new(vec![0, 1, 3, 1, 2, 3]);

    move |registry: &mut GlobalRegistry| {
        let Some(entities) = registry.entities.query::<CSprite>() else {
            return Ok(());
        };

        let shader = registry
            .resources
            .get_asset(&shader_id)
            .ok_or(QPError::AssetNotFound("shader".into()))?;

        let camera = registry
            .resources
            .get_camera::<Camera2D>(&camera_id)
            .ok_or(QPError::CameraNotFound)?;

        gl_enable(GLCapability::AlphaBlending);
        gl_blending_func(
            GLBlendingFactor::SrcAlpha,
            GLBlendingFactor::OneMinusSrcAlpha,
        );

        renderer.reset_info();
        renderer.begin_batch();
        for (entity, sprite) in entities.iter() {
            if sprite.is_none()
                || registry.entities.get::<CSkip>(&entity).is_some()
                || registry.entities.get::<CTransform2D>(&entity).is_none()
                || registry.entities.get::<CQuad>(&entity).is_none()
            {
                continue;
            }

            let quad = registry.entities.get::<CQuad>(&entity).unwrap();
            let transform = registry.entities.get::<CTransform2D>(&entity).unwrap();
            let model = transform.to_matrix();

            let mvp = camera.projection.0 * camera.view.0 * model;

            let texture = match registry.entities.get::<CTextureAtlas>(&entity) {
                Some(atlas) => registry.resources.get_asset(&atlas.texture),
                _ => None,
            };

            renderer.draw(
                vertices(
                    &mvp,
                    &texture,
                    glm::vec4(1.0, 1.0, 1.0, 1.0),
                    quad.positions(),
                ),
                shader,
                texture,
            );
        }
        renderer.end_batch();
        renderer.flush_batch(shader);

        Ok(())
    }
}

pub fn vertices(
    mvp: &glm::Mat4,
    texture: &Option<&TextureAsset>,
    color: glm::Vec4,
    positions: [glm::Vec4; 4],
) -> [Vertex; 4] {
    let pos1 = mvp * positions[0];
    let pos2 = mvp * positions[1];
    let pos3 = mvp * positions[2];
    let pos4 = mvp * positions[3];

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
