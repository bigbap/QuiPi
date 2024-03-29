use crate::{
    platform::opengl::capabilities::{gl_blending_func, gl_enable, GLBlendingFactor, GLCapability},
    prelude::{
        qp_assets::{RCamera2D, RShader},
        qp_ecs::components::{CSprite, CTransform2D},
        GlobalRegistry, QPError, Renderer, World,
    },
    QPResult,
};

use super::super::batch_renderer::BatchRenderer;

pub struct SpriteRenderer {
    camera: u64,
    shader: u64,

    renderer: BatchRenderer<10000, CSprite>,
}

impl SpriteRenderer {
    pub fn new(registry: &mut GlobalRegistry, camera: &str, shader: &str) -> QPResult<Self> {
        let Some(camera) = registry.asset_manager.get_asset_id(camera) else {
            return Err(QPError::CameraNotLoaded);
        };

        let Some(shader) = registry.asset_manager.get_asset_id(shader) else {
            return Err(QPError::ShaderNotLoaded);
        };

        Ok(Self {
            camera,
            shader,
            renderer: BatchRenderer::new(),
        })
    }
}

impl Renderer for SpriteRenderer {
    fn draw(&mut self, world: &mut World) -> Option<u32> {
        let entities = world.registry.entity_manager.query_all::<CSprite>();

        gl_enable(GLCapability::AlphaBlending);
        gl_blending_func(
            GLBlendingFactor::SrcAlpha,
            GLBlendingFactor::OneMinusSrcAlpha,
        );

        if world
            .registry
            .asset_manager
            .get::<RShader>(self.shader)
            .is_none()
        {
            #[cfg(debug_assertions)]
            println!("[sprite controller] tried to use a shader that is not loaded");

            return None;
        };

        let Some(camera) = world.registry.asset_manager.get::<RCamera2D>(self.camera) else {
            #[cfg(debug_assertions)]
            println!("[sprite controller] tried to use a camera that is not loaded");

            return None;
        };

        self.renderer.reset_info();
        self.renderer.begin_batch();
        for entity in entities.iter() {
            let Some(sprite) = world.registry.entity_manager.get::<CSprite>(&entity) else {
                #[cfg(debug_assertions)]
                println!("[sprite controller] tried to render a sprite without a sprite component");

                continue;
            };

            if sprite.skip {
                continue;
            }

            let Some(transform) = world.registry.entity_manager.get::<CTransform2D>(&entity) else {
                #[cfg(debug_assertions)]
                println!(
                    "[sprite controller] tried to render a sprite without a tranform component"
                );

                continue;
            };
            let model = transform.to_matrix();

            let Some(sprite) = world.registry.entity_manager.get_mut::<CSprite>(&entity) else {
                continue;
            };
            sprite.apply_matrices(model, camera.view, camera.projection);

            let sprite = world
                .registry
                .entity_manager
                .get::<CSprite>(&entity)
                .unwrap();
            let texture = match &sprite.texture_atlas {
                Some(atlas) => world.registry.asset_manager.get(atlas.texture),
                _ => None,
            };

            self.renderer.draw_mesh(
                sprite,
                world.registry.asset_manager.get(self.shader)?,
                texture,
            );
        }
        self.renderer.end_batch();
        self.renderer
            .flush_batch(world.registry.asset_manager.get(self.shader)?);

        Some(self.renderer.draw_calls)
    }
}
