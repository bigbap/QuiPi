extern crate sdl2;
extern crate gl;
extern crate nalgebra_glm as glm;
extern crate freetype as ft;
extern crate serde;
// extern crate gltf;

mod core;
mod data;
mod ecs;
mod editor;
mod gfx;
mod physics;
mod platform;
mod profiling;
mod quipi;
mod registry;
mod resource_manager;
mod schemas;

pub mod prelude {
    use super::*;

    pub use self::core::prelude as qp_core;
    pub use self::data::prelude as qp_data;
    pub use self::ecs::prelude as qp_ecs;

    #[cfg(feature = "qp_editor")]
    pub use self::editor::prelude as qp_editor;

    pub use self::gfx::prelude as qp_gfx;
    pub use self::physics::prelude as qp_physics;
    pub use self::profiling::prelude as qp_profiling;
    pub use self::schemas::prelude as qp_schemas;

    pub use quipi::QuiPi;
    pub use self::registry::Registry;
    pub use self::qp_ecs::VersionedIndex;

    // 3rd party - TODO: abstract this
    pub use sdl2::event::Event;
}
