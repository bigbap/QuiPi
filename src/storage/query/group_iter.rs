use crate::storage::{
    indexed_array::{Index, Iter},
    prelude::Component,
    query::Group,
};

pub trait GroupIter<G: Group + 'static> {
    fn step(&mut self) -> Option<(Index, G)>;
}

impl<C0: Component + 'static> GroupIter<&'static C0> for Iter<'static, C0> {
    fn step(&mut self) -> Option<(Index, &'static C0)> {
        let i1 = self;

        loop {
            let v = i1.next();
            if v.is_none() {
                return None;
            }

            match v {
                Some(Some((index, Some(entry)))) => return Some((index, entry)),
                _ => continue,
            }
        }
    }
}
