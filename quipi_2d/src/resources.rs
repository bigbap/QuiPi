pub mod camera;
pub mod tilemap;

pub use camera::RCamera2D;
pub use tilemap::RTileMap;

pub use quipi_core::resources::*;

use quipi_core::Registry;

pub fn register_resources(registry: &mut Registry) {
    registry
        .register_resource::<RCamera2D>()
        .register_resource::<RTileMap>();

    quipi_core::resources::register_resources(registry);
}