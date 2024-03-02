use crate::{
    qp_gfx::ShaderUniforms,
    qp_gfx::SpriteRenderer,
    qp_schemas::{load_scene_2d, SchemaScene2D, SchemaShader, SchemaTexture},
    App, GlobalRegistry, Schema,
};
use quipi::{
    app::{Controller, FrameResult},
    prelude::{
        qp_assets::RFont,
        qp_gfx::{QPText, QPTextStyle},
        QPError,
    },
    world::World,
};
use sdl2::{event::Event, keyboard::Keycode};

use super::{
    bubble::BubbleController,
    camera::{camera_schema, CameraController},
};

pub struct SceneController {}

impl SceneController {
    pub fn load(app: &mut App) -> Result<Self, QPError> {
        let scene = load_scene_2d("bubbles", scene_schema(&app.world))?;

        scene.build_entity(&mut app.world.registry)?;

        let camera_controller = CameraController::new(&mut app.world.registry)?;
        let bubble_controller = BubbleController::new(&mut app.world, camera_controller.camera)?;
        let text_controller = DebugInfoText::new(&mut app.world.registry)?;

        let renderer = SpriteRenderer::new(&mut app.world.registry, "main_camera", "sprite")?;

        app.register_controller(bubble_controller);
        app.register_controller(camera_controller);
        app.register_controller(text_controller);

        app.register_renderer(renderer);

        Ok(Self {})
    }
}

impl Controller for SceneController {
    fn update(&mut self, world: &mut World) -> FrameResult {
        for event in world.events.iter() {
            match event {
                Event::Quit { .. } => {
                    return FrameResult::Quit;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    if cfg!(debug_assertions) {
                        world.debug_mode = !world.debug_mode;
                    }
                }
                _ => (),
            };
        }

        FrameResult::None
    }
}

fn scene_schema(world: &World) -> SchemaScene2D {
    let (_x, _y, width, height) = world.viewport.get_dimensions();
    SchemaScene2D {
        name: "bubbles".to_string(),
        cameras: vec![camera_schema(width as f32, height as f32)],
        sprites: vec![],
        shaders: vec![SchemaShader {
            name: "sprite".to_string(),
            uniforms: vec![
                ShaderUniforms::ViewMatrix("view".into()),
                ShaderUniforms::ProjectionMatrix("projection".into()),
            ],
        }],
        textures: vec![SchemaTexture {
            name: "Bubble.png".into(),
            texture_dims: glm::vec2(1.0, 1.0),
        }],
    }
}

struct DebugInfoText {
    font: u64,
}

impl DebugInfoText {
    pub fn new(registry: &mut GlobalRegistry) -> Result<Self, QPError> {
        let font = registry
            .asset_manager
            .load_asset("Poppins-Regular", RFont::new("Poppins-Regular")?)?;

        Ok(Self { font })
    }
}

impl Controller for DebugInfoText {
    fn update(&mut self, world: &mut World) -> FrameResult {
        let entity_count = world.registry.entity_manager.count();
        let style = QPTextStyle {
            font: self.font,
            color: glm::vec4(0.1, 0.1, 0.1, 1.0),
            scale: 0.4,
        };
        world.text_buffer.push(QPText {
            text: format!("entities: {}", entity_count),
            pos: glm::vec2(20.0, 20.0),
            style: style.clone(),
        });
        world.text_buffer.push(QPText {
            text: format!("draw calls: {}", world.debug_info.draw_calls),
            pos: glm::vec2(20.0, 40.0),
            style: style.clone(),
        });
        world.text_buffer.push(QPText {
            text: format!("render ms: {}", world.debug_info.render_ms),
            pos: glm::vec2(20.0, 60.0),
            style: style.clone(),
        });
        world.text_buffer.push(QPText {
            text: format!("controller ms: {}", world.debug_info.controller_ms),
            pos: glm::vec2(20.0, 80.0),
            style: style.clone(),
        });
        world.text_buffer.push(QPText {
            text: format!("fps: {}", world.debug_info.fps as u32),
            pos: glm::vec2(20.0, 100.0),
            style: style.clone(),
        });
        world.text_buffer.push(QPText {
            text: format!("ms: {}", world.debug_info.frame_ms as u32),
            pos: glm::vec2(20.0, 120.0),
            style: style.clone(),
        });

        FrameResult::None
    }
}
