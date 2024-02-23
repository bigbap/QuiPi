extern crate sdl2;
extern crate gl;
extern crate nalgebra_glm as glm;
extern crate freetype as ft;
extern crate serde;
// extern crate gltf;

mod core;
mod data;
mod ecs;
mod egui;
mod gfx;
mod physics;
mod platform;
mod profiling;
mod quipi;
mod registry;
mod schemas;

pub mod prelude {
    use super::*;

    pub use self::core::api as core;
    pub use self::data::api as data;
    pub use self::ecs::api as ecs;
    pub use self::gfx::api as gfx;
    pub use self::physics::api as physics;
    pub use self::profiling::api as profiling;
    pub use self::schemas::api as schemas;

    pub use quipi::QuiPi;
    pub use self::egui::GUI;
    pub use self::registry::Registry;
    pub use self::ecs::VersionedIndex;

    // 3rd party - TODO: abstract this
    pub use sdl2::event::Event;
}
