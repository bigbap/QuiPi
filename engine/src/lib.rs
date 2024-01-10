pub extern crate sdl2;
pub extern crate gl;
pub extern crate nalgebra_glm as glm;
pub extern crate gltf;

pub mod core;
pub mod engine;
pub mod registry;
pub mod components;
pub mod resources;
pub mod systems;

pub use core::gfx;
pub use core::ecs;
pub use core::utils;
pub use core::VersionedIndex;
pub use engine::Game;
pub use registry::Registry;
pub use core::Component;
pub use core::ECS;
