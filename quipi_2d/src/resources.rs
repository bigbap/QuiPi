pub mod camera;

pub use camera::RCamera2D;

pub use quipi_core::resources::*;

use quipi_core::Registry;

pub fn register_resources(registry: &mut Registry) {
    registry
        .register_resource::<RCamera2D>();

    quipi_core::resources::register_resources(registry);
}