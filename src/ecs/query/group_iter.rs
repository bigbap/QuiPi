use crate::ecs::{
    indexed_array::{Index, Iter},
    prelude::Component,
    query::Group,
};

use super::GroupResult;

pub trait GroupIter<G: Group> {
    fn nexer(self) -> Box<dyn FnMut() -> Option<(Index, Box<dyn GroupResult>)>>;
}

impl<C0: Component> GroupIter<C0> for Iter<'static, C0> {
    fn nexer(mut self) -> Box<dyn FnMut() -> Option<(Index, Box<dyn GroupResult>)>> {
        Box::new(move || loop {
            let v = self.next();
            if v.is_none() {
                return None;
            }

            match v {
                Some(Some((index, entry))) => return Some((index, Box::<&C0>::new(entry))),
                _ => continue,
            }
        })
    }
}

// impl<C0: Component, C1: Component> GroupIter<(C0, C1)> for (Iter<'static, C0>, Iter<'static, C1>) {
//     fn nexer(&mut self) -> Option<(Index, Box<dyn GroupResult>)> {
//         let (i1, i2) = self;

//         loop {
//             let v1 = i1.next();
//             let v2 = i2.next();
//             if v1.is_none() || v2.is_none() {
//                 return None;
//             }

//             let v1 = v1.unwrap();
//             let v2 = v2.unwrap();
//             if v1.is_none() || v2.is_none() {
//                 continue;
//             }

//             let (index, e1) = v1.unwrap();
//             let (index, e2) = v2.unwrap();
//             if e1.is_none() || e2.is_none() {
//                 continue;
//             }

//             return Some((index, Box::new((e1.unwrap(), e2.unwrap()))));
//         }
//     }
// }

// pub trait GroupIter<G: Group> {
//     fn step(&mut self) -> Option<(Index, Box<dyn GroupResult>)>;
// }

// impl<'a, C0: Component> GroupIter<C0> for Iter<'a, C0> {
//     fn step(&mut self) -> Option<(Index, Box<dyn GroupResult>)> {
//         let i1 = self;

//         loop {
//             let v = i1.next();
//             if v.is_none() {
//                 return None;
//             }

//             match v {
//                 Some(Some((index, Some(entry)))) => return Some((index, Box::<&C0>::new(entry))),
//                 _ => continue,
//             }
//         }
//     }
// }

// impl<C0: Component, C1: Component> GroupIter<(C0, C1)> for (Iter<'static, C0>, Iter<'static, C1>) {
//     fn step(&mut self) -> Option<(Index, Box<dyn GroupResult>)> {
//         let (i1, i2) = self;

//         loop {
//             let v1 = i1.next();
//             let v2 = i2.next();
//             if v1.is_none() || v2.is_none() {
//                 return None;
//             }

//             let v1 = v1.unwrap();
//             let v2 = v2.unwrap();
//             if v1.is_none() || v2.is_none() {
//                 continue;
//             }

//             let (index, e1) = v1.unwrap();
//             let (index, e2) = v2.unwrap();
//             if e1.is_none() || e2.is_none() {
//                 continue;
//             }

//             return Some((index, Box::new((e1.unwrap(), e2.unwrap()))));
//         }
//     }
// }
