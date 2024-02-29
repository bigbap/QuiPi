extern crate quipi;
extern crate nalgebra_glm as glm;

use controllers::scene::SceneController;
pub use quipi::prelude::*;

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

mod controllers;

#[cfg(debug_assertions)]
#[cfg(feature = "qp_editor")]
mod editor;

pub fn run() -> Result<(), QPError> {
    let mut app = App::init(
        "Tiles",
        WIDTH,
        HEIGHT,
    )?;

    let scene = SceneController::load(&mut app)?;

    app.register_controller(scene);

    #[cfg(debug_assertions)]
    #[cfg(feature = "qp_editor")]
    app.register_controller(editor::AppEditor::new()?);

    app.run((0.3, 0.3, 0.3, 1.0))
}


fn main() {
    if let Err(e) = run() {
        eprintln!("Tiles ended unexpectedly: {}", e);
    };
}
