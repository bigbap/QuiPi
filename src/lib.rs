extern crate freetype as ft;
extern crate gl;
extern crate nalgebra_glm as glm;
extern crate sdl2;
extern crate serde;
// extern crate gltf;

pub mod app;
// pub mod asset_manager;
pub mod audio;
pub mod commands;
pub mod common;
pub mod core;
pub mod errors;
pub mod gfx;
pub mod physics;
pub mod platform;
pub mod plugin;
pub mod query;
pub mod resources;
pub mod storage;
// pub mod schemas;
pub mod schedule;
pub mod world;

// #[cfg(feature = "qp_editor")]
// mod editor;

#[cfg(feature = "qp_profiling")]
mod profiling;

type QPResult<T> = Result<T, errors::QPError>;

pub mod prelude {
    use super::*;

    // pub use self::asset_manager::assets as qp_assets;
    pub use self::audio::QPAudio as qp_audio;
    pub use self::common::prelude as qp_common;
    pub use self::core::prelude as qp_core;
    pub use self::gfx::prelude as qp_gfx;
    pub use self::storage::prelude as qp_storage;
    // pub use self::physics::prelude as qp_physics;
    // pub use self::schemas::prelude as qp_schemas;

    pub use self::app::App;
    pub use self::app::Controller;
    pub use self::app::FrameResult;
    pub use self::errors::QPError;
    pub use self::qp_storage::Index;
    // pub use self::qp_gfx::Renderer;
    // pub use self::schemas::prelude::Schema;
    pub use self::commands::*;
    pub use self::query::*;
    pub use self::schedule::*;
    pub use self::world::World;

    // #[cfg(feature = "qp_editor")]
    // pub use self::editor::prelude as qp_editor;

    #[cfg(feature = "qp_profiling")]
    pub use self::profiling::QPProfiler;

    // 3rd party - TODO: abstract this
    pub use sdl2::event::Event;

    pub use qp_common::plugins::default_plugins;
    pub use qp_common::plugins::render_plugins;
    pub use qp_common::resources::*;

    pub use plugin::*;
    pub use world::*;
}
