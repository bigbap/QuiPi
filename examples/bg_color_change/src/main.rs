pub use engine::{
    self,
    Game,
    Registry,
    VersionedIndex
};

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

struct MyGame {
    registry: Registry,
    timer: std::time::Instant,

    scene: VersionedIndex
}

impl MyGame {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = create_registry()?;
        let scene = create_scene(&mut registry)?;
        let timer = std::time::Instant::now();

        Ok(Self {
            registry,
            timer,
            scene
        })
    }

    pub fn ticks(&self) -> f32 {
        self.timer.elapsed().as_millis() as f32 / 1000.0
    }
}

impl Game for MyGame {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
        
        let bg_color = systems::get_color(
            _ticks,
            &self.scene,
            &mut self.registry.components
        );
        unsafe {
            gl::ClearColor(bg_color.0, bg_color.1, bg_color.2, bg_color.3);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        Some(())
    }
}

fn create_registry() -> Result<Registry, Box<dyn std::error::Error>> {
    let mut registry = Registry::init()?;

    registry
        .register_component::<components::ColorComponent>();

    Ok(registry)
}

fn create_scene(registry: &mut Registry) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    let scene = registry.components
        .create_entity()?;
    registry.components
        .add_component(
            &scene,
            components::ColorComponent(0.3, 0.3, 0.3, 1.0)
        );

    Ok(scene)
}

fn main() {
    let mut my_game = MyGame::new().expect("there was a problem initializing the game");

    if let Err(e) = engine::engine::run(&mut my_game, "Learn OpenGL", 800, 600) {
        eprintln!("{e}")
    }
}
