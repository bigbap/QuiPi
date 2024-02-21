pub mod indexed_array;
pub mod entity_manager;
pub mod query;
pub mod resources;
pub mod components;

pub use indexed_array::VersionedIndex;
pub use indexed_array::VersionedIndexAllocator;
pub use indexed_array::IndexedArray;
pub use component_derive::Component;
pub use entity_manager::EntityManager;
pub use entity_manager::ECSError;
pub use query::EMQuery;

pub trait Component {}
impl Component for () {}

mod tests;
