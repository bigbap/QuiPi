pub mod error;
pub mod ecs;
pub mod indexed_array;

pub use error::ECSError;
pub use ecs::ECS;
pub use indexed_array::VersionedIndex;
pub use indexed_array::VersionedIndexAllocator;
pub use indexed_array::IndexedArray;

pub use component_derive::Component;
pub trait Component {
    fn my_type(&self) -> String;
}
