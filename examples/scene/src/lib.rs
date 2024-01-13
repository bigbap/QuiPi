extern crate engine;
extern crate gl;
extern crate nalgebra_glm as glm;

use engine::{resources::{
    texture::TextureType,
    Shader, Camera3D
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
    camera: VersionedIndex,

    last_frame: f32
}

impl MyGame {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = create_registry()?;
        let timer = std::time::Instant::now();
        let camera = create_camera(&mut registry)?;

        Ok(Self {
            registry,
            timer,

            crates: vec![],
            camera,
            
            last_frame: timer.elapsed().as_millis() as f32 / 1000.0
        })
    }

    pub fn ticks(&self) -> f32 {
        self.timer.elapsed().as_millis() as f32 / 1000.0
    }
}

impl engine::Game for MyGame {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // let lights = create_lights(&mut self.registry)?;
        
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
            self.camera,
            vec![
                diffuse,
                specular
            ],
        )?;

        Ok(())
    }

    fn handle_frame(&mut self, event_pump: &mut EventPump) -> Option<()> {
        let ticks = self.ticks();
        let delta = ticks - self.last_frame;
        
        self.last_frame = ticks;
        
        let camera = self.registry.get_resource_mut::<Camera3D>(&self.camera).unwrap();
        let speed = 5.0 * delta;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => return None,
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => engine::gfx::view::adjust_viewport_dims(w, h),
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => return None,
                    Some(Keycode::W) => camera.position += camera.front * speed,
                    Some(Keycode::S) => camera.position -= camera.front * speed,
                    Some(Keycode::A) => camera.position -= camera.right() * speed,
                    Some(Keycode::D) => camera.position += camera.right() * speed,
                    _ => ()
                },
                _event => ()
            };
        }

        for entity in self.crates.iter() {
            engine::gfx::buffer::clear_buffer(None);

            systems::update_entity(entity, &self.registry);
            systems::draw(
                entity,
                &self.registry,
            ).expect("there was a problem drawing the entity");
        }

        Some(())
    }
}

