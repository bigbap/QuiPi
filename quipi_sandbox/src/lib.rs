use std::collections::HashMap;

use quipi::{
    engine::QuiPiApp,
    Registry,
    core::{
        random::Random,
        time::now_secs
    },
    resources::{
        register_resources,
        Shader
    },
    components::{
        register_components,
        CEulerAngles,
    },
    systems::scene::{
        save_scene,
        load_scene
    },
    wrappers::{
        opengl::shader::ShaderProgram,
        sdl2::window::QuiPiWindow
    },
    AppState,
    FrameResponse,
    sdl2::{
        event::Event,
        keyboard::Keycode
    }, schema::SchemaScene
};

extern crate quipi;
extern crate nalgebra_glm as glm;

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

pub type SandboxError = Box<dyn std::error::Error>;

pub struct QuiPiSandbox {
    registry: quipi::Registry,
    rand: Random,

    scene: Option<SchemaScene>,
}

impl QuiPiSandbox {
    pub fn new() -> Result<Self, SandboxError> {
        let mut registry = Registry::init()?;
        let rand = Random::from_seed(now_secs()?);

        register_resources(&mut registry);
        register_components(&mut registry);

        Ok(Self {
            registry,
            rand,
            scene: None,
        })
    }
}

impl QuiPiApp for QuiPiSandbox {
    fn init(
        &mut self,
        window: &QuiPiWindow
    ) -> Result<(), SandboxError> {
        let scene = load_scene("start")?;

        scene.build_scene(&mut self.registry)?;

        // for (key, value) in scene.shaders.iter() {
        //     let shader = ShaderProgram::new(key)?;
        //
        //     self.registry.create_resource(key, Shader {
        //         program: shader,
        //         uniforms: value.to_vec()
        //     })?;
        // }

        // for camera in scene.cameras.iter() {
        //     let renderer = Renderer2D::new(
        //         &mut self.registry,
        //         &camera.tag,
        //         camera.params,
        //         camera.transform.clone(),
        //         CEulerAngles::default()
        //     )?;
        //     renderer.update_view_matrix(&mut self.registry);
        //
        //     self.renderers.insert(camera.tag.clone(), Box::new(renderer));
        // }

        // for entity in scene.entities.iter() {
        //     self.registry.create_entity(&entity.tag.tag)?
        // }

        window.relative_mouse_mode(false);

        self.scene = Some(scene);

        Ok(())
    }

    fn handle_frame(
        &mut self,
        app_state: &mut AppState
    ) -> Result<FrameResponse, SandboxError> {
        

        for event in app_state.events.iter() {
            match event {
                Event::Quit { .. } => {
                    if let Some(scene) = &self.scene {
                        save_scene("start", scene)?;
                    }

                    return Ok(FrameResponse::Quit)
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => app_state.editor_mode = !app_state.editor_mode,
                _ => ()
            }
        }

        Ok(FrameResponse::None)
    }
}
