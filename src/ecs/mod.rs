mod bundle;
mod components;
mod entity_manager;
mod indexed_array;
mod query;
mod tests;

pub mod prelude {
    use super::*;

    pub use macros::Component;
    pub trait Component {}
    impl Component for () {}

    pub use indexed_array::Allocator;
    pub use indexed_array::Index;
    pub use indexed_array::IndexedArray;

    pub use components::components;
    pub use entity_manager::EntityBuilder;
    pub use entity_manager::EntityManager;
    pub use query::EMQuery;
}
