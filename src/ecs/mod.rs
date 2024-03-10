mod bundle;
mod entity_manager;
mod indexed_array;

pub mod prelude {
    use super::*;

    pub use bundle::*;
    pub use macros::Component;

    pub use indexed_array::Allocator;
    pub use indexed_array::Index;
    pub use indexed_array::IndexedArray;

    pub use entity_manager::EntityManager;
}
