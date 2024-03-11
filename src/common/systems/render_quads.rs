use crate::{
    common::{
        assets::{ShaderAsset, TextureAsset, TextureCoords},
        bundles::CSprite,
        prelude::components::{CColor, CQuad, CSkip, CTexture, CTransform2D},
        resources::{AssetId, Camera2D, CameraId},
    },
    platform::opengl::capabilities::{gl_blending_func, gl_enable, GLBlendingFactor, GLCapability},
    prelude::{
        qp_gfx::{BatchRenderer, Vertex},
        QPError, System,
    },
};

const BATCH_SIZE: usize = 10000;

pub fn render_quads(shader_id: AssetId, camera_id: CameraId) -> impl System {
    debug_assert!(shader_id.validate::<ShaderAsset>());

    let mut renderer = BatchRenderer::<BATCH_SIZE, 4>::new(vec![0, 1, 3, 1, 2, 3]);

    move |world: &mut crate::prelude::World| {
        let query = world.query();
        let Some(entities) = query.entities::<CSprite>() else {
            return Ok(());
        };

        let shader = query
            .asset(&shader_id)
            .ok_or(QPError::AssetNotFound("shader".into()))?;

        let camera = query
            .camera::<Camera2D>(&camera_id)
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
                || query.entity::<CSkip>(&entity).is_some()
                || query.entity::<CTransform2D>(&entity).is_none()
                || query.entity::<CQuad>(&entity).is_none()
            {
                continue;
            }

            let quad = query.entity::<CQuad>(&entity).unwrap();
            let transform = query.entity::<CTransform2D>(&entity).unwrap();
            let model = transform.to_matrix();

            let mvp = camera.projection.0 * camera.view.0 * model;

            let texture = match query.entity::<CTexture>(&entity) {
                Some(atlas) => query.asset(&atlas.id),
                _ => None,
            };

            let color = query
                .entity::<CColor>(&entity)
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
