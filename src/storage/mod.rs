mod bundle;
mod indexed_array;
mod manager;

pub mod prelude {
    use super::*;

    pub use bundle::*;
    pub use macros::Component;

    pub use indexed_array::Allocator;
    pub use indexed_array::Index;
    pub use indexed_array::IndexedArray;

    pub use manager::*;
}
