use crate::{
    qp_gfx::SpriteRenderer,
    qp_schemas::{
        SchemaScene2D,
        SchemaShader,
        SchemaTexture,
        load_scene_2d
    },
    qp_data::{
        FrameResponse,
        FrameState,
        IController,
        ISchema,
        ShaderUniforms
    },
    App,
    GlobalRegistry
};
use quipi::prelude::{qp_assets::RFont, qp_gfx::{QPText, QPTextStyle}, QPError};
use sdl2::{
    event::Event,
    keyboard::Keycode
};

use super::{
    camera::{camera_schema, CameraController},
    player::PlayerController,
    tiles::TileControler
};

pub struct SceneController {}

impl SceneController {
    pub fn load(engine: &mut App) -> Result<Self, QPError> {
        let scene = load_scene_2d(
            "tile_map",
            scene_schema()
        )?;

        scene.build_entity(&mut engine.registry)?;

        let tile_controller = TileControler::new(&mut engine.registry)?;
        let player_controller = PlayerController::new(
            &mut engine.registry,
            tile_controller.tile_map
        )?;
        let camera_controller = CameraController::new(
            player_controller.player,
            &mut engine.registry
        )?;
        let text_controller = DebugInfoText::new(&mut engine.registry)?;

        engine.register_controller(tile_controller);
        engine.register_controller(player_controller);
        engine.register_controller(camera_controller);
        engine.register_controller(text_controller);

        let renderer = SpriteRenderer::new(
            &mut engine.registry,
            "main_camera",
            "sprite"
        )?;

        engine.register_renderer(renderer);

        Ok(Self {})
    }
}

impl IController for SceneController {
    fn update(&mut self, frame_state: &mut FrameState, _registry: &mut GlobalRegistry) -> FrameResponse {
        for event in frame_state.events.iter() {
            match event {
                Event::Quit {..} => {
                    return FrameResponse::Quit;
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    if cfg!(debug_assertions) {
                        frame_state.debug_mode = !frame_state.debug_mode;
                    }
                },
                _ => ()
            };
        }
    
        FrameResponse::None
    }
}

fn scene_schema() -> SchemaScene2D {
    SchemaScene2D {
        name: "bouncing_shapes".to_string(),
        cameras: vec![camera_schema()],
        sprites: vec![],
        shaders: vec![SchemaShader {
            name: "sprite".to_string(),
            uniforms: vec![
                ShaderUniforms::ViewMatrix("view".into()),
                ShaderUniforms::ProjectionMatrix("projection".into())
            ]
        }],
        textures: vec![
            SchemaTexture {
                name: "Bubble.png".into(),
                texture_dims: glm::vec2(1.0, 1.0)
            },
            SchemaTexture {
                name: "Player.png".into(),
                texture_dims: glm::vec2(1.0, 1.0)
            },
            SchemaTexture {
                name: "tiles.png".into(),
                texture_dims: glm::vec2(1.0, 2.0)
            }
        ]
    }
}

struct DebugInfoText {
    font: u64
}

impl DebugInfoText {
    pub fn new(
        registry: &mut GlobalRegistry
    ) -> Result<Self, QPError> {
        let font = registry.asset_manager.load_asset(
            "Poppins-Regular".to_string(),
            RFont::new("Poppins-Regular")?
        )?;

        Ok(Self {
            font
        })
    }
}

impl IController for DebugInfoText {
    fn update(
        &mut self,
        frame_state: &mut FrameState,
        registry: &mut GlobalRegistry
    ) -> FrameResponse {
        let entity_count = registry.entity_manager.count();
        let style = QPTextStyle {
            font: self.font,
            color: glm::vec4(1.0, 1.0, 1.0, 1.0),
            scale: 0.6
        };
        frame_state.text_buffer.push(QPText {
            text: format!("entities: {}", entity_count),
            pos: glm::vec2(20.0, 20.0),
            style: style.clone()
        });
        frame_state.text_buffer.push(QPText {
            text: format!("draw calls: {}", frame_state.debug_info.draw_calls),
            pos: glm::vec2(20.0, 40.0),
            style: style.clone()
        });
        frame_state.text_buffer.push(QPText {
            text: format!("render ms: {}", frame_state.debug_info.render_ms),
            pos: glm::vec2(20.0, 60.0),
            style: style.clone()
        });
        frame_state.text_buffer.push(QPText {
            text: format!("controller ms: {}", frame_state.debug_info.controller_ms),
            pos: glm::vec2(20.0, 80.0),
            style: style.clone()
        });
        frame_state.text_buffer.push(QPText {
            text: format!("fps: {}", frame_state.debug_info.fps as u32),
            pos: glm::vec2(20.0, 100.0),
            style: style.clone()
        });
        frame_state.text_buffer.push(QPText {
            text: format!("ms: {}", frame_state.debug_info.frame_ms as u32),
            pos: glm::vec2(20.0, 120.0),
            style: style.clone()
        });

        FrameResponse::None
    }
}