mod components;
mod entity_manager;
mod indexed_array;
mod query;
mod tests;

pub mod prelude {
    use super::*;

    pub use component_derive::Component;
    pub trait Component {}
    impl Component for () {}

    pub use indexed_array::IndexedArray;
    pub use indexed_array::VersionedIndex;
    pub use indexed_array::VersionedIndexAllocator;

    pub use components::components;
    pub use entity_manager::EntityBuilder;
    pub use entity_manager::EntityManager;
    pub use query::EMQuery;
}
