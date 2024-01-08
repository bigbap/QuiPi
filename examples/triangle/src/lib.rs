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
                    unsafe { gl::Viewport(0, 0, w, h); }
                },

                Event::KeyDown { keycode, .. } if keycode.is_some_and(|k| k == Keycode::Escape) => {
                    return None
                },
                _event => ()
            };
        }
        
        let mut bg_color = (0.0, 0.0, 0.0, 1.0);

        if let Some(index) = self.active_scene {
            let scene = &self.scenes[index];

            systems::update_entities(scene, &self.registry);

            bg_color = systems::get_color(
                _ticks,
                scene,
                &mut self.registry
            );

            systems::draw(scene, &mut self.registry).expect("there was a problem drawing the scene");
        }

        unsafe {
            gl::ClearColor(bg_color.0, bg_color.1, bg_color.2, bg_color.3);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
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
    
    let shader = registry.create_resource(resources::Shader::new("simple")?)?;

    let config = ObjConfig {
        positions: vec![
            -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0,  0.5, 0.0
        ],
        // colors: vec![
        //     1.0, 0.0, 0.0,
        //     0.0, 1.0, 0.0,
        //     0.0, 0.0, 1.0
        // ],
        indices: vec![
            0, 1, 2
        ],
        ..ObjConfig::default()
    };

    registry.create_entity()
        .with(components::DrawComponent { shader_id: shader })
        .with(components::ColorComponent(0.0, 0.0, 0.0, 1.0))
        .with(engine::gfx::MeshComponent::new(&config)?)
        .done()
}
