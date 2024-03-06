use core::fmt;
use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

const DEFAULT_CAPACITY: usize = 4;

/// https://github.com/fitzgen/generational-arena/blob/master/src/lib.rs
/// https://www.youtube.com/watch?v=aKLntZcp27M
/// https://kyren.github.io/2018/09/14/rustconf-talk.html
///
/// inspiration from:
/// - RustConf 2018 - Closing Keynote - Using Rust For Game Development by Catherine West
/// - https://github.com/fitzgen/generational-arena

#[derive(Debug, Default, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct VersionedIndex {
    index: usize,
    version: u64,
}

impl fmt::Display for VersionedIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.index, self.version)
    }
}

#[derive(Debug, Clone, Copy)]
enum AllocatorEntry {
    Occupied { version: u64 },
    Free { next: Option<usize> },
}

impl Default for AllocatorEntry {
    fn default() -> Self {
        Self::Free { next: None }
    }
}

#[derive(Debug)]
pub struct VersionedIndexAllocator {
    entries: Vec<AllocatorEntry>,
    next: Option<usize>,
    version: u64,
    length: usize,
}

impl Default for VersionedIndexAllocator {
    fn default() -> Self {
        Self {
            entries: Vec::<AllocatorEntry>::with_capacity(DEFAULT_CAPACITY),
            next: None,
            version: 0,
            length: 0,
        }
    }
}

impl VersionedIndexAllocator {
    pub fn allocate(&mut self) -> VersionedIndex {
        let index = match self.try_allocate() {
            None => {
                let i = self.grow();

                VersionedIndex {
                    index: i,
                    version: self.version,
                }
            }
            Some(index) => index,
        };

        self.entries[index.index] = AllocatorEntry::Occupied {
            version: self.version,
        };

        self.length += 1;

        index
    }

    pub fn deallocate(&mut self, index: VersionedIndex) {
        if self.validate(&index) {
            self.entries[index.index] = AllocatorEntry::Free { next: self.next };

            self.next = Some(index.index);
            self.version += 1;

            self.length -= 1;
        }
    }

    pub fn is_allocated(&self, index: &VersionedIndex) -> bool {
        match self.entries.get(index.index) {
            None => false,
            Some(entry) => match entry {
                AllocatorEntry::Occupied { version } => *version == index.version,
                AllocatorEntry::Free { .. } => false,
            },
        }
    }

    pub fn length(&self) -> usize {
        self.entries.len()
    }
    pub fn valid_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| matches!(e, AllocatorEntry::Occupied { .. }))
            .count()
    }

    fn try_allocate(&mut self) -> Option<VersionedIndex> {
        match self.next {
            Some(i) => match self.entries[i] {
                AllocatorEntry::Occupied { .. } => panic!("corrupt indexed array"),
                AllocatorEntry::Free { next } => {
                    self.next = next;

                    Some(VersionedIndex {
                        index: i,
                        version: self.version,
                    })
                }
            },
            None => None,
        }
    }

    fn grow(&mut self) -> usize {
        if self.entries.len() > self.entries.capacity() {
            self.entries.reserve(self.entries.capacity() * 2);
        }

        self.entries.push(AllocatorEntry::Free { next: None });

        self.entries.len() - 1
    }

    // pub fn reset(&mut self) {
    //     self.entries.clear();
    //     self.next = None;
    //     self.version = 0;
    //     self.length = 0;
    // }

    pub fn validate(&self, index: &VersionedIndex) -> bool {
        match self.entries.get(index.index) {
            Some(AllocatorEntry::Occupied { version }) => *version == index.version,
            _ => false,
        }
    }
}

#[derive(Debug, Default)]
pub struct Entry<T> {
    value: T,
    version: u64,
}

#[derive(Debug)]
pub struct IndexedArray<T> {
    allocator: Rc<RefCell<VersionedIndexAllocator>>,
    list: Vec<Option<Entry<T>>>,
}

impl<T> IndexedArray<T> {
    pub fn new(allocator: Rc<RefCell<VersionedIndexAllocator>>) -> Self {
        Self {
            allocator,
            list: Vec::<Option<Entry<T>>>::with_capacity(DEFAULT_CAPACITY),
        }
    }

    pub fn set(&mut self, index: &VersionedIndex, value: T) {
        let i = index.index;

        if i >= self.list.capacity() {
            self.list.reserve(self.list.capacity() * 2);
        }

        if i >= self.list.len() {
            self.list.resize_with(i + 4, || None);
        }

        self.list[i] = Some(Entry {
            version: index.version,
            value,
        });
    }

    pub fn unset(&mut self, index: &VersionedIndex) {
        let i = index.index;

        if i >= self.list.len() {
            return;
        }

        self.list[i] = None;
    }

    pub fn get(&self, index: &VersionedIndex) -> Option<&T> {
        match self.list.get(index.index) {
            Some(Some(entry)) => {
                if entry.version == index.version {
                    Some(&entry.value)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn get_mut(&mut self, index: &VersionedIndex) -> Option<&mut T> {
        match self.list.get_mut(index.index) {
            None => None,
            Some(None) => None,
            Some(Some(entry)) => {
                if entry.version == index.version {
                    Some(&mut entry.value)
                } else {
                    None
                }
            }
        }
    }

    /// TODO: write test
    pub fn get_entities(&self) -> Vec<VersionedIndex> {
        self.list
            .iter()
            .enumerate()
            .filter_map(|(i, wrapped)| match wrapped {
                Some(entry) => {
                    let index = VersionedIndex {
                        index: i,
                        version: entry.version,
                    };

                    match self.allocator.borrow().validate(&index) {
                        true => Some(index),
                        false => None,
                    }
                }
                None => None,
            })
            .collect()
    }

    pub fn get_indexed_entry(&self, index: usize) -> Option<IndexedEntry<&T>> {
        match self.list.get(index) {
            Some(Some(entry)) => Some(IndexedEntry {
                index: VersionedIndex {
                    index,
                    version: entry.version,
                },
                entry: &entry.value,
            }),
            Some(&None) => None,
            None => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IndexedEntry<T> {
    pub index: VersionedIndex,
    pub entry: T,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct Entity(String);

    type EntityMap<T> = IndexedArray<T>;

    #[test]
    fn indexed_array_getting_setting_removing() {
        let allocator = Rc::new(RefCell::new(VersionedIndexAllocator::default()));
        let mut entities = EntityMap::<Entity>::new(allocator.clone());

        let player_id = allocator.borrow_mut().allocate();
        let npc_id = allocator.borrow_mut().allocate();
        let enemy_id = allocator.borrow_mut().allocate();

        entities.set(&player_id, Entity("player".to_string()));
        entities.set(&npc_id, Entity("npc".to_string()));
        entities.set(&enemy_id, Entity("enemy".to_string()));

        assert_eq!(allocator.borrow().length(), 3);

        let mut bullets = Vec::<VersionedIndex>::new();
        for _ in 0..3 {
            let bullet = allocator.borrow_mut().allocate();

            entities.set(&bullet, Entity("bullet".to_string()));

            bullets.push(bullet);
        }

        assert_eq!(allocator.borrow().length(), 6);
        assert_eq!(
            entities.get(&player_id),
            Some(&Entity("player".to_string()))
        );
        assert_eq!(entities.get(&enemy_id), Some(&Entity("enemy".to_string())));
        assert_eq!(entities.get(&npc_id), Some(&Entity("npc".to_string())));

        // npc_id is no longer valid after this call because the remove function destroys it
        allocator.borrow_mut().deallocate(npc_id);

        assert_eq!(allocator.borrow().valid_count(), 5);

        for bullet in bullets {
            allocator.borrow_mut().deallocate(bullet);
        }

        assert_eq!(allocator.borrow().valid_count(), 2);
        assert_eq!(
            entities.get(&player_id),
            Some(&Entity("player".to_string()))
        );
        assert_eq!(entities.get(&enemy_id), Some(&Entity("enemy".to_string())));
    }

    #[test]
    fn indexed_array_versioning() {
        let allocator = Rc::new(RefCell::new(VersionedIndexAllocator::default()));
        let mut entities = EntityMap::<Entity>::new(allocator.clone());

        let player_id = allocator.borrow_mut().allocate();
        let npc_id = allocator.borrow_mut().allocate();
        let enemy_id = allocator.borrow_mut().allocate();

        entities.set(&player_id, Entity("player".to_string()));
        entities.set(&npc_id, Entity("npc".to_string()));
        entities.set(&enemy_id, Entity("enemy".to_string()));

        assert_eq!(
            entities.get(&VersionedIndex {
                index: 1,
                version: 0
            }),
            Some(&Entity("npc".to_string()))
        );

        allocator.borrow_mut().deallocate(npc_id);

        // used to hold npc
        assert!(!allocator.borrow().is_allocated(&VersionedIndex {
            index: 1,
            version: 0
        }));

        let npc_id = allocator.borrow_mut().allocate();
        entities.set(&npc_id, Entity("npc".to_string()));

        assert_eq!(
            entities.get(&VersionedIndex {
                index: 1,
                version: 1
            }),
            Some(&Entity("npc".to_string()))
        );

        assert_eq!(
            entities.get(&VersionedIndex {
                index: 1,
                version: 0
            }),
            None
        );

        // version 1 is allocated while version 0 is not
        assert!(!allocator.borrow().is_allocated(&VersionedIndex {
            index: 1,
            version: 0
        }));
        assert!(allocator.borrow().is_allocated(&VersionedIndex {
            index: 1,
            version: 1
        }));
    }
}
