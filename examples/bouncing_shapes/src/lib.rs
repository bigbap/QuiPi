extern crate quipi_2d;
extern crate nalgebra_glm as glm;

use controllers::scene::SceneController;
pub use quipi_2d::{
    IController,
    components::{
        CScene,
        CRGBA,
        CTransform2D
    },
    core::canvas,
    schemas::{
        ISchema,
        SchemaScene2D,
        SchemaCamera2D,
        camera2d::DEFAULT_CAMERA
    },
    systems::scene::load_scene_2d,
    FrameResponse,
    FrameState,
    QuiPiWindow,
    Registry,
    VersionedIndex
};
pub use quipi_core::opengl::textures::*;

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

mod controllers;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = quipi_2d::QuiPi2D::init(
        "Bouncing Shapes",
        WIDTH,
        HEIGHT,
    )?;

    let scene = SceneController::load(&mut app)?;

    app.register_controller(scene);
    app.run((1.0, 1.0, 0.8, 1.0))
    // app.run((0.3, 0.3, 0.3, 1.0))
}
