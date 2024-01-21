extern crate engine;
extern crate nalgebra_glm as glm;

use std::vec;

use engine::{
    gfx::ElementArrayMesh,
    VersionedIndex,
    components::{CModelNode, CModelMatrix},
    resources::Shader
};
use sdl2::{
    EventPump,
    keyboard::Keycode,
    event::{
        Event,
        WindowEvent
    }
};

mod config;

pub use config::CONFIG;

pub struct MyGame {
    registry: engine::Registry,
    shader: Option<VersionedIndex>,
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
            shader: None,
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
        self.shader = Some(self.registry.create_resource(
            engine::resources::Shader::new(
                &format!("{}/shaders/simple", CONFIG.asset_path),
                vec![]
            )?
        )?);

        Ok(())
    }

    fn handle_frame(
        &mut self,
        event_pump: &mut EventPump
    ) -> Result<Option<()>, Box<dyn std::error::Error>> {
        let _ticks = self.ticks();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => return Ok(None),
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    engine::gfx::view::adjust_viewport_dims(w, h)
                },

                Event::KeyDown { keycode, .. } if keycode.is_some_and(|k| k == Keycode::Escape) => {
                    return Ok(None)
                },
                _event => ()
            };
        }

        if let Some(index) = self.active_scene {
            let scene = &self.scenes[index];

            engine::gfx::buffer::clear_buffer(Some((0.3, 0.4, 0.5, 1.0)));

            let shader = self.registry.get_resource::<Shader>(&self.shader.unwrap()).unwrap();
            engine::systems::draw::s_draw_entity(
                scene,
                &self.registry,
                shader,
                &glm::Mat4::identity()
            );
        }

        Ok(Some(()))
    }
}

fn create_registry() -> Result<engine::Registry, Box<dyn std::error::Error>> {
    let mut registry = engine::Registry::init()?;

    engine::resources::register_resources(&mut registry);
    engine::components::register_components(&mut registry);

    Ok(registry)
}

fn create_scene(
    registry: &mut engine::Registry
) -> Result<engine::VersionedIndex, Box<dyn std::error::Error>> {
    type ObjConfig = engine::systems::load_obj::ObjectConfig;
    

    let config = ObjConfig {
        points: vec![
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

    let mesh = ElementArrayMesh::new(&config.indices)?;
    mesh
        .create_vbo_at(&config.points, 0, 3)?
        .create_vbo_at(&config.colors, 1, 3)?;

    registry.create_entity("triangle")?
        .with(engine::components::CModelNode {
            mesh: Some(mesh),
            ..CModelNode::default()
        })?
        .with(CModelMatrix::default())?
        .done()
}
