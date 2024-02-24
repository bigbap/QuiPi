use crate::{
    qp_gfx::Renderer2D,
    qp_schemas::{
        SchemaScene2D,
        SchemaShader,
        SchemaTexture,
        load_scene_2d
    },
    qp_core::Timer,
    qp_data::{
        FrameResponse,
        FrameState,
        IController,
        IRenderer,
        ISchema,
        ShaderUniforms
    },
    QuiPi,
    Registry
};
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
    pub fn load(engine: &mut QuiPi) -> Result<Self, Box<dyn std::error::Error>> {
        let scene = load_scene_2d(
            "tile_map",
            scene_schema()
        )?;

        scene.build_entity(&mut engine.registry)?;

        let tile_controller = TileControler::new(&mut engine.registry)?;

        let player_controller = PlayerController::new(&mut engine.registry, tile_controller.tile_map)?;
        let camera_controller = CameraController::new(
            player_controller.player,
            &mut engine.registry
        )?;

        engine.register_controller(tile_controller);
        engine.register_controller(player_controller);
        engine.register_controller(camera_controller);

        let renderer = Renderer2D::new(
            &mut engine.registry,
            "main_camera",
            "sprite"
        )?;

        engine.register_renderer(renderer);
        engine.register_renderer(DebugInfoText::new());

        Ok(Self {})
    }
}

impl IController for SceneController {
    fn update(&mut self, frame_state: &mut FrameState, _registry: &mut Registry) -> FrameResponse {
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
    timer: Timer
}

impl DebugInfoText {
    pub fn new() -> Self {
        Self {
            timer: Timer::new()
        }
    }
}

impl IRenderer for DebugInfoText {
    fn draw(
        &mut self,
        frame_state: &mut FrameState,
        registry: &mut Registry
    ) -> Option<u32> {
        self.timer.delta();

        // draw the entity count
        let entity_count = registry.entities.count();
        frame_state.text_render.color = glm::vec4(0.9, 0.9, 0.9, 1.0);
        frame_state.text_render.scale = 0.6;
        frame_state.text_render.draw(
            format!("entities: {}", entity_count),
            glm::vec2(20.0, 20.0)
        );
        frame_state.text_render.draw(
            format!("draw calls: {}", frame_state.debug_info.draw_calls),
            glm::vec2(20.0, 40.0)
        );
        frame_state.text_render.draw(
            format!("render ms: {}", frame_state.debug_info.render_ms),
            glm::vec2(20.0, 60.0)
        );
        frame_state.text_render.draw(
            format!("controller ms: {}", frame_state.debug_info.controller_ms),
            glm::vec2(20.0, 80.0)
        );
        frame_state.text_render.draw(
            format!("fps: {}", frame_state.debug_info.fps as u32),
            glm::vec2(20.0, 100.0)
        );
        frame_state.text_render.draw(
            format!("ms: {}", frame_state.debug_info.frame_ms as u32),
            glm::vec2(20.0, 120.0)
        );

        Some(6)
    }
}