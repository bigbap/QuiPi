extern crate quipi;
extern crate nalgebra_glm as glm;

use controllers::scene::SceneController;
pub use quipi::prelude::*;

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

mod controllers;
#[cfg(debug_assertions)]
mod editor;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = QuiPi::init(
        "Bouncing Shapes",
        WIDTH,
        HEIGHT,
    )?;

    let scene = SceneController::load(&mut app)?;
    #[cfg(debug_assertions)]
    let editor = editor::AppEditor::new()?;

    app.register_controller(scene);
    #[cfg(debug_assertions)]
    app.register_controller(editor);
    app.run((0.8, 0.8, 0.4, 1.0))
}


fn main() {
    if let Err(e) = run() {
        eprintln!("Bouncing Shapes ended unexpectedly: {}", e);
    };
}
