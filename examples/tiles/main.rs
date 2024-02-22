extern crate quipi;
extern crate nalgebra_glm as glm;

use controllers::scene::SceneController;
pub use quipi::{
    IController,
    ecs::{
        components::*,
        resources::*
    },
    canvas,
    schemas::{
        ISchema,
        SchemaScene2D,
        SchemaCamera2D,
        camera2d::DEFAULT_CAMERA
    },
    scene::load_scene_2d,
    FrameResponse,
    FrameState,
    Registry,
    VersionedIndex,
    QuiPi
};

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

mod controllers;
mod editor;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = QuiPi::init(
        "Tiles",
        WIDTH,
        HEIGHT,
    )?;

    let scene = SceneController::load(&mut app)?;
    let editor = editor::AppEditor::new()?;

    app.register_controller(scene);
    app.register_controller(editor);
    app.run((0.3, 0.3, 0.3, 1.0))
}


fn main() {
    if let Err(e) = run() {
        eprintln!("Tiles ended unexpectedly: {}", e);
    };
}
