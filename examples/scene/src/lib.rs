extern crate engine;
extern crate gl;
extern crate nalgebra_glm as glm;

use engine::{resources::{
    texture::TextureType,
    Shader
}, VersionedIndex};
use sdl2::{
    EventPump,
    keyboard::Keycode,
    event::{
        Event,
        WindowEvent
    }
};

mod components;
mod systems;
mod resources;
mod config;
mod scene;

pub use config::CONFIG;

use scene::*;

type Crate = VersionedIndex;

pub struct MyGame {
    registry: engine::Registry,
    timer: std::time::Instant,
    
    crates: Vec<Crate>,
}

impl MyGame {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let registry = create_registry()?;
        let timer = std::time::Instant::now();

        Ok(Self {
            registry,
            timer,

            crates: vec![],
        })
    }

    pub fn ticks(&self) -> f32 {
        self.timer.elapsed().as_millis() as f32 / 1000.0
    }
}

impl engine::Game for MyGame {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // let lights = create_lights(&mut self.registry)?;
        let camera = create_camera(&mut self.registry)?;
        
        let shader_program = Shader::new(&format!("{}shaders/lighting", CONFIG.asset_path))?;
        let shader = self.registry.create_resource(
            shader_program
        )?;
        
        let diffuse = create_texture(
            &mut self.registry,
            &format!("{}objects/textures/container.png", CONFIG.asset_path),
            TextureType::Diffuse
        )?;
        let specular = create_texture(
            &mut self.registry,
            &format!("{}objects/textures/container_specular.png", CONFIG.asset_path),
            TextureType::Specular
        )?;

        self.crates = create_crates(
            &mut self.registry,
            shader,
            camera,
            vec![
                diffuse,
                specular
            ],
        )?;

        Ok(())
    }

    fn handle_frame(&mut self, event_pump: &mut EventPump) -> Option<()> {
        let _ticks = self.ticks();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => return None,
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    unsafe { gl::Viewport(0, 0, w, h); }
                },

                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => return None,
                    Some(Keycode::W) => (),
                    Some(Keycode::A) => (),
                    Some(Keycode::S) => (),
                    Some(Keycode::D) => (),
                    _ => ()
                },
                _event => ()
            };
        }

        for entity in self.crates.iter() {
            engine::gfx::buffer::clear_buffer(None);

            systems::update_entity(entity, &self.registry);
            systems::draw_ebo(
                entity,
                &self.registry,
            ).expect("there was a problem drawing the entity");
        }

        Some(())
    }
}

