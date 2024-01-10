pub extern crate sdl2;
pub extern crate gl;
pub extern crate nalgebra_glm as glm;
pub extern crate gltf;

pub mod core;
pub mod ecs;
pub mod engine;
pub mod registry;
pub mod gfx;
pub mod components;
pub mod resources;

pub use core::VersionedIndex;
pub use engine::Game;
pub use registry::Registry;
pub use ecs::Component;
pub use ecs::ComponentRegistry;
