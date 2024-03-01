extern crate freetype as ft;
extern crate gl;
extern crate nalgebra_glm as glm;
extern crate sdl2;
extern crate serde;
// extern crate gltf;

pub mod app;
pub mod asset_manager;
pub mod audio;
pub mod core;
pub mod data;
pub mod ecs;
pub mod errors;
pub mod gfx;
pub mod physics;
pub mod platform;
pub mod registry;
pub mod schemas;

#[cfg(feature = "qp_editor")]
mod editor;

#[cfg(feature = "qp_profiling")]
mod profiling;

type QPResult<T> = Result<T, errors::QPError>;

pub mod prelude {
    use super::*;

    pub use self::asset_manager::assets as qp_assets;
    pub use self::audio::QPAudio as qp_audio;
    pub use self::core::prelude as qp_core;
    pub use self::data::prelude as qp_data;
    pub use self::ecs::prelude as qp_ecs;
    pub use self::gfx::prelude as qp_gfx;
    pub use self::physics::prelude as qp_physics;
    pub use self::schemas::prelude as qp_schemas;

    pub use self::app::App;
    pub use self::app::FrameResult;
    pub use self::errors::QPError;
    pub use self::qp_data::FrameState;
    pub use self::qp_data::IController;
    pub use self::qp_data::IRenderer;
    pub use self::qp_ecs::VersionedIndex;
    pub use self::registry::GlobalRegistry;

    #[cfg(feature = "qp_editor")]
    pub use self::editor::prelude as qp_editor;

    #[cfg(feature = "qp_profiling")]
    pub use self::profiling::QPProfiler;

    // 3rd party - TODO: abstract this
    pub use sdl2::event::Event;
}
