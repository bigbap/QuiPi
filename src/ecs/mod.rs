mod bundle;
mod components;
mod entity_manager;
mod indexed_array;
mod tests;

pub mod prelude {
    use super::*;

    pub use bundle::Component;
    pub use bundle::ComponentId;
    pub use macros::Component;

    pub use indexed_array::Allocator;
    pub use indexed_array::Index;
    pub use indexed_array::IndexedArray;

    pub use components::components;
    pub use entity_manager::EntityManager;
}
