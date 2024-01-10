extern crate engine;
extern crate gl;
extern crate nalgebra_glm as glm;

use std::vec;

use engine::gfx::object_loader::{
    load_obj_file,
    ObjectConfig
};
use engine::components::texture::TextureType;
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

pub use config::CONFIG;

pub struct MyGame {
    registry: engine::Registry,
    timer: std::time::Instant,

    entities: Vec<engine::VersionedIndex>,
}

impl MyGame {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let registry = create_registry()?;
        let timer = std::time::Instant::now();

        Ok(Self {
            registry,
            timer,
            entities: vec![],
        })
    }

    pub fn ticks(&self) -> f32 {
        self.timer.elapsed().as_millis() as f32 / 1000.0
    }
}

impl engine::Game for MyGame {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.entities.append(&mut create_crate(&mut self.registry)?);

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

                Event::KeyDown { keycode, .. } if keycode.is_some_and(|k| k == Keycode::Escape) => {
                    return None
                },
                _event => ()
            };
        }

        for entity in self.entities.iter() {
            engine::gfx::buffer::clear_buffer(None);

            systems::update_entity(entity, &self.registry);

            systems::draw_ebo(entity, &self.registry)
                .expect("there was a problem drawing the scene");
        }

        Some(())
    }
}

fn create_registry() -> Result<engine::Registry, Box<dyn std::error::Error>> {
    let mut registry = engine::Registry::init()?;

    resources::register_resources(&mut registry);
    components::register_components(&mut registry);

    Ok(registry)
}

fn create_crate(
    registry: &mut engine::Registry
) -> Result<Vec<engine::VersionedIndex>, Box<dyn std::error::Error>> {
    use components::*;

    let shader = registry.create_resource(
        resources::Shader::new(&format!("{}shaders/simple", CONFIG.asset_path))?
    )?;

    // load the object data
    let (models_obj, _materials_obj) = load_obj_file(format!("{}objects/crate.obj", CONFIG.asset_path))?;
    let model_configs = ObjectConfig::from_obj(models_obj)?;

    let mut entities = vec![];
    for config in model_configs {
        entities.push(registry.create_entity()
            .with(DrawComponent { shader_id: shader })
            .with(MeshComponent::new(&config)?)
            .with(TextureComponent { id: 4647, kind: TextureType::Diffuse })
            .with(TransformComponent {
                translate: None,
                scale: Some(glm::vec3(0.5, 0.5, 0.5)),
                rotate: Some(glm::vec3(0.2, 0.3, 0.0)),
                angle: Some(0.1)
            })
            .done()?
        )
    }

    Ok(entities)
}
