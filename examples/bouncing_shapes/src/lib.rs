extern crate skald;
extern crate nalgebra_glm as glm;

use skald::{
    Registry,
    VersionedIndex,
    resources::{
        register_resources,
        Shader,
        shader::UniformVariable
    },
    gfx::{
        clear_buffer,
        ShaderProgram,
    },
    builders::camera::build_ortho_camera,
    math::random::Random,
    utils::{now_secs, Timer},
    systems::{
        mvp_matrices::s_set_view_matrix,
        rotation::s_rotate_camera
    },
    components::{
        register_components,
        CEulerAngles,
        CTransform,
        CBoundingBox
    },
    core::{
        GUI,
        text::{
            TextRenderer,
            DEFAULT_FONT
        },
    }
};

pub static WIDTH: u32 = 800;
pub static HEIGHT: u32 = 600;

mod systems;

use systems::*;

pub struct MyGame {
    registry: Registry,
    timer: Timer,
    rand: Random,
    
    shader: Option<VersionedIndex>,
    camera: VersionedIndex,
    debug_gui: Option<GUI>,
    text_renderer: Option<TextRenderer>,
}

impl MyGame {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = Registry::init()?;
        let rand = Random::from_seed(now_secs()?);

        register_resources(&mut registry);
        register_components(&mut registry);

        let camera = build_ortho_camera(
            &mut registry,
            CBoundingBox {
                right: WIDTH as f32,
                top: HEIGHT as f32,
                near: 0.0,
                far: 0.2,
                ..CBoundingBox::default()
            },
            CTransform {
                translate: glm::vec3(0.0, 0.0, 0.0),
                ..CTransform::default()
            },
            CEulerAngles {
                pitch: 0.0,
                yaw: 90.0,
                roll: 0.0
            }
        )?;

        s_rotate_camera(&mut registry, &camera);
        s_set_view_matrix(&camera, &mut registry);

        Ok(MyGame {
            registry,
            shader: None,
            text_renderer: None,
            timer: Timer::new()?,
            rand,
            camera,
            debug_gui: None
        })
    }
}

impl skald::Game for MyGame {
    fn init(&mut self, debug_gui: Option<GUI>) -> Result<(), Box<dyn std::error::Error>> {
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
        
        self.shader = Some(shader_id);
        self.debug_gui = debug_gui;

        let mut text = TextRenderer::new(
            DEFAULT_FONT,
            WIDTH as f32,
            HEIGHT as f32
        )?;
        text.color = glm::vec3(1.0, 1.0, 1.0);
        text.scale = 0.7;
        self.text_renderer = Some(text);
        
        Ok(())
    }

    fn handle_frame(
        &mut self,
        event_pump: &mut sdl2::EventPump
    ) -> Result<Option<()>, Box<dyn std::error::Error>> {
        let delta = self.timer.delta();

        // handle input events
        for event in event_pump.poll_iter() {
            let response = s_handle_input(
                &mut self.registry,
                event,
                &mut self.rand
            )?;

            if response.is_none() { return Ok(None) }
        }

        s_update(
            &mut self.registry,
            delta,
        )?;

        // render
        clear_buffer((0.2, 0.0, 0.0, 1.0));

        s_draw_frame(
            &self.registry,
            &self.shader.unwrap(),
            &self.camera,
            self.text_renderer.as_ref().unwrap()
        )?;

        // update gui
        if let Some(debug_gui) = &mut self.debug_gui {
            debug_gui.update()?;
        }

        Ok(Some(()))
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

