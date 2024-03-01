mod indexed_array;
mod entity_manager;
mod query;
mod components;
mod tests;

pub mod prelude {
    use super::*;

    pub use component_derive::Component;
    pub trait Component {}
    impl Component for () {}

    pub use indexed_array::VersionedIndex;
    pub use indexed_array::VersionedIndexAllocator;
    pub use indexed_array::IndexedArray;
    
    pub use entity_manager::EntityManager;
    pub use query::EMQuery;
    pub use components::components;
}