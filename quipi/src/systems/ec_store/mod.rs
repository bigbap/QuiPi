pub mod indexed_array;
pub mod entity_manager;

pub use indexed_array::VersionedIndex;
pub use indexed_array::VersionedIndexAllocator;
pub use indexed_array::IndexedArray;
pub use component_derive::Component;
pub use entity_manager::EntityManager;
pub use entity_manager::EMError;

pub trait Component {
    fn my_type(&self) -> String;
}

#[macro_export]
macro_rules! get_components {
    ($em:expr, $entity:expr, $($cmp:ty,)+) => {
        (
            $entity.to_string(),
            $(
                (std::any::type_name::<$cmp>().split("::").last().unwrap(), $em.get::<$cmp>($entity)),
            )?
        )
    };
}

mod tests;
