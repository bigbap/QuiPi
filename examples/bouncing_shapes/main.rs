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

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = QuiPi::init(
        "Bouncing Shapes",
        WIDTH,
        HEIGHT,
    )?;

    let scene = SceneController::load(&mut app)?;

    app.register_controller(scene);
    app.run((0.3, 0.3, 0.3, 1.0))
}


fn main() {
    if let Err(e) = run() {
        eprintln!("Bouncing Shapes ended unexpectedly: {}", e);
    };
}
