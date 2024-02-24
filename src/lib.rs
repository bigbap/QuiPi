extern crate sdl2;
extern crate gl;
extern crate nalgebra_glm as glm;
extern crate freetype as ft;
extern crate serde;
// extern crate gltf;

mod audio;
mod core;
mod data;
mod ecs;
#[cfg(feature = "qp_editor")]
mod editor;
mod errors;
mod gfx;
mod physics;
mod platform;
#[cfg(feature = "qp_profiling")]
mod profiling;
mod quipi;
mod registry;
mod resource_manager;
mod schemas;

type QPResult<T> = Result<T, errors::QPError>;

pub mod prelude {
    use super::*;

    pub use self::audio::prelude as qp_audio;
    pub use self::core::prelude as qp_core;
    pub use self::data::prelude as qp_data;
    pub use self::ecs::prelude as qp_ecs;

    #[cfg(feature = "qp_editor")]
    pub use self::editor::prelude as qp_editor;

    pub use self::gfx::prelude as qp_gfx;
    pub use self::physics::prelude as qp_physics;
    pub use self::schemas::prelude as qp_schemas;

    pub use self::quipi::QuiPi;
    pub use self::errors::QPError;
    pub use self::registry::Registry;
    pub use self::qp_ecs::VersionedIndex;

    #[cfg(feature = "qp_profiling")]
    pub use self::profiling::QPProfiler;

    // 3rd party - TODO: abstract this
    pub use sdl2::event::Event;
}
