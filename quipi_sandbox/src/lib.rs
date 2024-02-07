use quipi::{
    engine::QuiPiApp,
    Registry,
    components::CRGBA,
    systems::{
        scene::load_scene,
        rendering::canvas,
    },
    wrappers::sdl2::window::QuiPiWindow,
    FrameState,
    FrameResponse,
    schemas::{
        SchemaScene,
        ISchema
    },
};

extern crate quipi;
extern crate nalgebra_glm as glm;

mod input;
mod draw;
mod update;

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

pub type SandboxError = Box<dyn std::error::Error>;

pub struct QuiPiSandbox {
    scene: Option<SchemaScene>,
}

impl QuiPiSandbox {
    pub fn new() -> Result<Self, SandboxError> {
        Ok(Self {
            scene: None,
        })
    }
}

impl QuiPiApp for QuiPiSandbox {
    fn init(
        &mut self,
        registry: &mut Registry,
        window: &QuiPiWindow
    ) -> Result<(), SandboxError> {
        let mut scene = load_scene("main", SchemaScene::default())?;
        scene.clr_color = CRGBA { r: 0.2, g: 0.2, b: 0.2, a: 1.0 };
        scene.build(registry)?;
        self.scene = Some(scene);

        window.relative_mouse_mode(false);

        Ok(())
    }

    fn handle_frame(
        &mut self,
        registry: &mut Registry,
        frame_state: &mut FrameState
    ) -> Result<FrameResponse, SandboxError> {
        if let Some(scene) = &self.scene {
            frame_state.clear_color = scene.clr_color;
        }

        update::update_frame(registry);

        draw::draw_frame(registry)?;
        draw_debug_info(registry, frame_state);

        input::handle_input(frame_state, &self.scene)
    }
}

fn draw_debug_info(registry: &Registry, app_state: &mut FrameState) {
    // draw the entity count
    let (_x, _y, width, height) = canvas::get_dimensions();
    let entity_count = registry.entities.count();
    app_state.text_render.color = glm::vec3(1.0, 1.0, 1.0);
    app_state.text_render.scale = 0.7;
    app_state.text_render.draw(
        format!("entities: {}", entity_count),
        glm::vec2(width as f32 - 120.0, height as f32 - 30.0)
    );
}

