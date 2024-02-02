extern crate quipi;
extern crate nalgebra_glm as glm;

use quipi::{
    Registry,
    VersionedIndex,
    resources::{
        register_resources,
        Shader,
        shader::UniformVariable
    },
    math::random::Random,
    utils::now_secs,
    systems::rendering::{
        IRenderer,
        Renderer2D,
        canvas
    },
    components::{
        register_components,
        CEulerAngles,
        CTransform,
        CBoundingBox
    },
    wrappers::{
        opengl::shader::ShaderProgram,
        egui::GUI, sdl2::window::QuiPiWindow,
    },
    AppState, FrameResponse
};

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

mod systems;

use systems::*;

pub struct MyGame {
    registry: Registry,
    rand: Random,
    
    shader: Option<VersionedIndex>,
    renderer: Renderer2D,
    debug_gui: Option<GUI>,
}

impl MyGame {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = Registry::init()?;
        let rand = Random::from_seed(now_secs()?);

        register_resources(&mut registry);
        register_components(&mut registry);

        let renderer = Renderer2D::new(
            &mut registry,
            CBoundingBox {
                right: WIDTH as f32,
                top: HEIGHT as f32,
                near: 0.0,
                far: 0.2,
                ..CBoundingBox::default()
            },
            CTransform::default(),
            CEulerAngles::default()
        )?;

        renderer.update_view_matrix(&mut registry);

        Ok(MyGame {
            registry,
            shader: None,
            rand,
            renderer,
            debug_gui: None
        })
    }
}

impl quipi::QuiPiApp for MyGame {
    fn init(
        &mut self,
        _winapi: &QuiPiWindow
    ) -> Result<(), Box<dyn std::error::Error>> {
        let shader = ShaderProgram::new("assets/shaders/shape")?;
        let shader_id = self.registry.create_resource(Shader {
            program: shader,
            uniforms: vec![
                UniformVariable::MVPMatrix("mvpMatrix".to_string())
            ]
        })?;

        create_shapes(
            &mut self.registry,
            &mut self.rand
        );
        
        let mut gui: Option<GUI> = None;
        if cfg!(debug_assertions) {
            gui = Some(GUI::new(1.0)?);
        }

        self.shader = Some(shader_id);
        self.debug_gui = gui;
        
        Ok(())
    }

    fn handle_frame(
        &mut self,
        app_state: &mut AppState
    ) -> Result<FrameResponse, Box<dyn std::error::Error>> {
        // handle input
        let frame_response = s_handle_input(
            app_state,
            &mut self.registry,
            &mut self.rand
        )?;

        s_update(
            app_state,
            &mut self.registry,
        )?;

        // render
        s_draw_frame(
            &mut self.registry,
            &self.shader.unwrap(),
            &self.renderer,
        )?;

        // // update gui
        // if let Some(debug_gui) = &mut self.debug_gui {
        //     debug_gui.update()?;
        // }

        // draw the entity count
        let (_x, _y, width, height) = canvas::get_dimensions();
        let entity_count = self.registry.entity_count();
        app_state.text_render.color = glm::vec3(1.0, 1.0, 1.0);
        app_state.text_render.scale = 0.7;
        app_state.text_render.draw(
            format!("entities: {}", entity_count),
            glm::vec2(width as f32 - 120.0, height as f32 - 30.0)
        );


        Ok(frame_response)
    }
}

fn create_shapes(
    registry: &mut Registry,
    rand: &mut Random
) {
    for _ in 0..10 {
        let _ = s_spawn_quad(registry, rand);
    }
}

