use crate::{
    platform::opengl::capabilities::{gl_blending_func, gl_enable, GLBlendingFactor, GLCapability},
    prelude::{
        qp_assets::{RCamera2D, RShader},
        qp_common::components::{CSprite, CTransform2D},
        GlobalRegistry, QPError,
    },
    QPResult,
};

use super::{super::batch_renderer::BatchRenderer, Renderer};

pub struct SpriteRenderer {
    camera: u64,
    shader: u64,

    renderer: BatchRenderer<10000, 4>,
}

impl SpriteRenderer {
    pub fn new(registry: &mut GlobalRegistry, camera: &str, shader: &str) -> QPResult<Self> {
        let Some(camera) = registry.assets.get_asset_id(camera) else {
            return Err(QPError::CameraNotLoaded);
        };

        let Some(shader) = registry.assets.get_asset_id(shader) else {
            return Err(QPError::ShaderNotLoaded);
        };

        Ok(Self {
            camera,
            shader,
            renderer: BatchRenderer::new(vec![0, 1, 3, 1, 2, 3]),
        })
    }
}

impl Renderer for SpriteRenderer {
    fn draw(&mut self, registry: &mut GlobalRegistry) -> Option<u32> {
        let entities = registry.entities.query_all::<CSprite>();

        gl_enable(GLCapability::AlphaBlending);
        gl_blending_func(
            GLBlendingFactor::SrcAlpha,
            GLBlendingFactor::OneMinusSrcAlpha,
        );

        if registry.assets.get::<RShader>(self.shader).is_none() {
            #[cfg(debug_assertions)]
            println!("[sprite controller] tried to use a shader that is not loaded");

            return None;
        };

        let Some(camera) = registry.assets.get::<RCamera2D>(self.camera) else {
            #[cfg(debug_assertions)]
            println!("[sprite controller] tried to use a camera that is not loaded");

            return None;
        };

        self.renderer.reset_info();
        self.renderer.begin_batch();
        for entity in entities.iter() {
            let Some(sprite) = registry.entities.get::<CSprite>(&entity) else {
                #[cfg(debug_assertions)]
                println!("[sprite controller] tried to render a sprite without a sprite component");

                continue;
            };

            if sprite.skip {
                continue;
            }

            let Some(transform) = registry.entities.get::<CTransform2D>(&entity) else {
                #[cfg(debug_assertions)]
                println!(
                    "[sprite controller] tried to render a sprite without a tranform component"
                );

                continue;
            };
            let model = transform.to_matrix();

            let Some(sprite) = registry.entities.get_mut::<CSprite>(&entity) else {
                continue;
            };
            sprite.apply_matrices(model, camera.view, camera.projection);

            let sprite = registry.entities.get::<CSprite>(&entity).unwrap();
            let texture = match &sprite.texture_atlas {
                Some(atlas) => registry.assets.get(atlas.texture),
                _ => None,
            };

            self.renderer.draw(
                sprite.vertices(),
                registry.assets.get(self.shader)?,
                texture,
            );
        }
        self.renderer.end_batch();
        self.renderer.flush_batch(registry.assets.get(self.shader)?);

        Some(self.renderer.draw_calls)
    }
}
