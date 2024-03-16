pub mod group;
pub mod group_arrays;
pub mod group_iter;

pub use group::*;
pub use group_iter::*;

use super::indexed_array::Index;

pub struct Query {
    pub nexer: Box<dyn FnMut() -> Option<(Index, Box<dyn GroupResult>)>>,
}

// impl Query {
//     pub fn new<G: Group>(iterators: Box<dyn GroupIter<G>>) -> Self {
//         let nexer = iterators.nexer();

//         Self { nexer }
//     }
// }

impl Iterator for Query {
    type Item = (Index, Box<dyn GroupResult>);

    fn next(&mut self) -> Option<Self::Item> {
        let val = (self.nexer)();

        match val {
            Some(entry) => return Some(entry),
            _ => {
                // let inner = std::mem::replace(&mut self.inner, Box::new(Dummy {}));

                // std::mem::drop(inner)
            }
        }

        None
    }
}

struct Dummy {}
impl<G: Group> GroupIter<G> for Dummy {
    fn nexer(mut self) -> Box<dyn FnMut() -> Option<(Index, Box<dyn GroupResult>)>> {
        Box::new(move || None)
    }
}
