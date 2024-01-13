extern crate engine;
extern crate gl;
extern crate nalgebra_glm as glm;

use std::vec;

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

    scenes: Vec<engine::VersionedIndex>,
    active_scene: Option<usize>
}

impl MyGame {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let registry = create_registry()?;
        let timer = std::time::Instant::now();

        Ok(Self {
            registry,
            timer,
            scenes: vec![],
            active_scene: None
        })
    }

    pub fn ticks(&self) -> f32 {
        self.timer.elapsed().as_millis() as f32 / 1000.0
    }
}

impl engine::Game for MyGame {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let scene = create_scene(&mut self.registry)?;
        
        self.scenes.push(scene);
        self.active_scene = Some(0);

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
                    engine::gfx::view::adjust_viewport_dims(w, h)
                },

                Event::KeyDown { keycode, .. } if keycode.is_some_and(|k| k == Keycode::Escape) => {
                    return None
                },
                _event => ()
            };
        }

        if let Some(index) = self.active_scene {
            let scene = &self.scenes[index];

            engine::gfx::buffer::clear_buffer(
                Some(systems::get_color(
                    _ticks,
                    scene,
                    &mut self.registry
                ))
            );

            systems::update_entities(scene, &self.registry);

            systems::draw_ebo(scene, &mut self.registry).expect("there was a problem drawing the scene");
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

fn create_scene(
    registry: &mut engine::Registry
) -> Result<engine::VersionedIndex, Box<dyn std::error::Error>> {
    type ObjConfig = engine::gfx::object_loader::ObjectConfig;
    
    let shader = registry.create_resource(
        resources::Shader::new(&format!("{}/shaders/simple", CONFIG.asset_path))?
    )?;

    let config = ObjConfig {
        positions: vec![
            -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0,  0.5, 0.0
        ],
        colors: vec![
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0
        ],
        indices: vec![
            0, 1, 2
        ],
        ..ObjConfig::default()
    };

    registry.create_entity()?
        .with(components::DrawComponent { shader_id: shader })?
        .with(components::ColorComponent(0.0, 0.0, 0.0, 1.0))?
        .with(engine::components::MeshComponent::new(&config)?)?
        .done()
}
