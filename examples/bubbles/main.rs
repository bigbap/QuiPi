extern crate nalgebra_glm as glm;
extern crate quipi;

use controllers::scene::SceneController;
pub use quipi::prelude::*;

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

mod controllers;

#[cfg(debug_assertions)]
#[cfg(feature = "qp_editor")]
mod editor;

pub fn run() -> Result<(), QPError> {
    let mut app = App::init("Bouncing Shapes", WIDTH, HEIGHT, 8576394876)?;

    let scene = SceneController::load(&mut app)?;
    app.register_controller(scene);

    #[cfg(debug_assertions)]
    #[cfg(feature = "qp_editor")]
    {
        let editor = editor::AppEditor::new()?;
        app.register_renderer(editor);
    }

    app.run((0.8, 0.8, 0.4, 1.0))
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Bouncing Shapes ended unexpectedly: {}", e);
    };
}
