use quipi_2d::{
    controllers::SpriteController,
    schemas::{
        ISchema,
        SchemaScene2D,
        SchemaShader, SchemaTexture
    },
    systems::scene::load_scene_2d,
    QuiPi2D
};
use quipi_core::{
    rendering::RenderInfo,
    resources::shader::UniformVariable,
    FrameResponse,
    FrameState,
    IController,
    Registry
};
use sdl2::{
    event::Event,
    keyboard::Keycode
};

use super::{bubble::BubbleController, camera::{camera_schema, CameraController}, player::PlayerController, tiles::TileControler};

pub struct SceneController {}

impl SceneController {
    pub fn load(engine: &mut QuiPi2D) -> Result<Self, Box<dyn std::error::Error>> {
        let scene = load_scene_2d(
            "bouncing_shapes",
            scene_schema()
        )?;

        scene.build_entity(&mut engine.registry)?;

        TileControler::new(&mut engine.registry)?;

        let player_controller = PlayerController::new(&mut engine.registry)?;
        let bubble_controller = BubbleController::new(&mut engine.registry)?;
        let camera_controller = CameraController::new(
            player_controller.player,
            &mut engine.registry
        )?;

        let sprite_controller = SpriteController::new(
            &mut engine.registry,
            "main_camera",
            "sprite"
        )?;

        engine.register_controller(player_controller);
        engine.register_controller(bubble_controller);
        
        engine.register_controller(sprite_controller);
        engine.register_controller(camera_controller);

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
                        frame_state.editor_mode = !frame_state.editor_mode;
                    }
                },
                _ => ()
            };
        }
    
        FrameResponse::None
    }

    fn draw(&mut self, frame_state: &mut FrameState, registry: &mut Registry) -> Option<RenderInfo> {
        // draw the entity count
        let entity_count = registry.entities.count();
        frame_state.text_render.color = glm::vec4(1.0, 1.0, 1.0, 1.0);
        frame_state.text_render.scale = 0.6;
        frame_state.text_render.draw(
            format!("entities: {}", entity_count),
            glm::vec2(20.0, 20.0)
        );
        frame_state.text_render.draw(
            format!("draw calls: {}", frame_state.render_info.num_draw_calls),
            glm::vec2(20.0, 40.0)
        );
        frame_state.text_render.draw(
            format!("fps: {}", frame_state.debug_info.fps as u32),
            glm::vec2(20.0, 60.0)
        );
        frame_state.text_render.draw(
            format!("ms: {}", frame_state.debug_info.ms as u32),
            glm::vec2(20.0, 80.0)
        );

        None
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
                UniformVariable::ViewMatrix("view".into()),
                UniformVariable::ProjectionMatrix("projection".into())
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
            }
        ]
    }
}