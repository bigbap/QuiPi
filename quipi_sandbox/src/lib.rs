extern crate quipi_2d as quipi;
extern crate nalgebra_glm as glm;

pub use quipi::{
    components::{
        CScene,
        CRGBA
    }, rendering::canvas, schemas::{
        ISchema,
        SchemaScene2D
    },
    systems::scene::load_scene_2d,
    FrameResponse,
    FrameState,
    QuiPiApp,
    QuiPiWindow,
    Registry,
    VersionedIndex
};

mod input;
mod update;

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

pub type SandboxError = Box<dyn std::error::Error>;

pub struct QuiPiSandbox {
    scene: Option<VersionedIndex>,
}

impl QuiPiSandbox {
    pub fn new() -> Self {
        Self {
            scene: None,
        }
    }
}

impl QuiPiApp for QuiPiSandbox {
    fn init(
        &mut self,
        registry: &mut Registry,
        window: &QuiPiWindow
    ) -> Result<(), SandboxError> {
        let scene_schema = load_scene_2d("main", SchemaScene2D {
            name: CScene { name: "main".to_string() },
            clr_color: CRGBA { value: [0.2, 0.2, 0.2, 1.0] },
            ..Default::default()
        })?;

        self.scene = Some(scene_schema.build(registry)?);

        window.relative_mouse_mode(false);

        Ok(())
    }

    fn handle_frame(
        &mut self,
        registry: &mut Registry,
        frame_state: &mut FrameState
    ) -> Result<FrameResponse, SandboxError> {
        if self.scene.is_none() {
            return Err("There is no scene defined".into());
        };
        
        let scene = self.scene.unwrap();

        if let Some(color) = registry.entities.get::<CRGBA>(&scene) {
            frame_state.clear_color = *color;
        }

        update::update_frame(registry);

        draw_debug_info(registry, frame_state);

        input::handle_input(frame_state, scene, &registry)
    }
}

fn draw_debug_info(registry: &Registry, app_state: &mut FrameState) {
    // draw the entity count
    let (_x, _y, width, height) = canvas::get_dimensions();
    let entity_count = registry.entities.count();
    app_state.text_render.color = glm::vec4(1.0, 1.0, 1.0, 1.0);
    app_state.text_render.scale = 0.7;
    app_state.text_render.draw(
        format!("entities: {}", entity_count),
        glm::vec2(width as f32 - 120.0, height as f32 - 30.0)
    );
}

