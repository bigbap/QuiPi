pub mod assets;
pub mod bundles;
pub mod components;
pub mod plugins;
pub mod resources;

pub mod prelude {
    pub use super::assets;
    pub use super::bundles;
    pub use super::components::components;
    pub use super::plugins;
    pub use super::resources;
}
