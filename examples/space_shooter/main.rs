extern crate nalgebra_glm as glm;
extern crate quipi;

pub use quipi::prelude::*;

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

pub fn run() -> Result<(), QPError> {
    let mut app = App::init("Space Shooter", WIDTH, HEIGHT)?;

    app.run((0.1, 0.1, 0.1, 1.0))
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Space Shooter ended unexpectedly: {}", e);
    }
}

pub struct GameController {}

impl Controller for GameController {
    fn update(&mut self, _world: &mut World) -> FrameResult {
        FrameResult::None
    }
}
