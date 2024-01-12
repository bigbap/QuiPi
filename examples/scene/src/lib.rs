extern crate engine;
extern crate gl;
extern crate nalgebra_glm as glm;

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

pub struct MyGame {
    registry: engine::Registry,
    timer: std::time::Instant,
    
    lights: Vec<engine::VersionedIndex>,
    entities: Vec<engine::VersionedIndex>,
    camera: engine::VersionedIndex,
}

impl MyGame {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = create_registry()?;
        let timer = std::time::Instant::now();

        let camera = create_camera(&mut registry)?;

        Ok(Self {
            registry,
            timer,

            lights: vec![],
            entities: vec![],
            camera
        })
    }

    pub fn ticks(&self) -> f32 {
        self.timer.elapsed().as_millis() as f32 / 1000.0
    }
}

impl engine::Game for MyGame {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.entities.append(
            &mut create_crate(
                &mut self.registry,
                self.camera
            )?
        );

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

        for entity in self.entities.iter() {
            engine::gfx::buffer::clear_buffer(None);

            systems::update_entity(entity, &self.registry);
            systems::draw_ebo(entity, &self.registry).expect("there was a problem drawing the entity");
        }

        Some(())
    }
}

