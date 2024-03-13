pub mod group;
pub mod group_arrays;
pub mod group_iter;

pub use group::*;
pub use group_arrays::*;
pub use group_iter::*;

use super::manager::Storage;

pub struct Query<G: Group> {
    storage: &'static Storage,
    inner: Box<dyn GroupIter<G>>,
}

impl<G: Group> Query<G> {
    // pub fn new(group: G) -> Self {

    // }
}

// impl<'a, G: Group<'a>> Query<'a, G> {
//     pub fn new(group: ) -> Self {
//         Self {
//             remaining: inner.len(),
//             allocator,
//             inner,
//         }
//     }
// }
