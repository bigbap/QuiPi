use super::{
    indexed_array::{Index, IndexedArray},
    prelude::Component,
};

pub struct Filtered<'a, C: Component + 'static> {
    inner: &'a IndexedArray<C>,
    count: u32,
}
