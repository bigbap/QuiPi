use serde::{Serialize, Deserialize};

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
    version: u64
}

#[derive(Debug, Clone, Copy)]
enum AllocatorEntry {
    Occupied { version: u64 },
    Free { next: Option<usize> }
}

impl Default for AllocatorEntry {
    fn default() -> Self {
        Self::Free { next: None }
    }
}

#[derive(Debug, Default)]
pub struct VersionedIndexAllocator {
    entries: Vec<AllocatorEntry>,
    next: Option<usize>,
    version: u64,
    length: usize
}

impl VersionedIndexAllocator {
    pub fn allocate(&mut self) -> VersionedIndex {
        let index = match self.try_allocate() {
            None => {
                let i = self.grow();

                VersionedIndex {
                    index: i,
                    version: self.version
                }
            },
            Some(index) => index
        };

        self.entries[index.index] = AllocatorEntry::Occupied {
            version: self.version
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
                AllocatorEntry::Free {..} => false
            }
        }
    }

    pub fn length(&self) -> usize { self.length }
    pub fn valid_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| matches!(e, AllocatorEntry::Occupied {..}))
            .count()
    }

    fn try_allocate(&mut self) -> Option<VersionedIndex> {
        match self.next {
            Some(i) => match self.entries[i] {
                AllocatorEntry::Occupied {..} => panic!("corrupt indexed array"),
                AllocatorEntry::Free { next } => {
                    self.next = next;

                    Some(VersionedIndex {
                        index: i,
                        version: self.version
                    })
                }
            },
            None => None
        }
    }

    fn grow(&mut self) -> usize {
        self.entries.push(
            AllocatorEntry::Free { next: None }
        );

        self.entries.len() - 1
    }

    // pub fn reset(&mut self) {
    //     self.entries.clear();
    //     self.next = None;
    //     self.version = 0;
    //     self.length = 0;
    // }

    pub fn validate(&self, index: &VersionedIndex) -> bool {
        let entity = self.entries[index.index];

        if let AllocatorEntry::Occupied { version } = entity {
            if version == index.version {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Default)]
pub struct Entry<T> {
    value: T,
    version: u64
}

#[derive(Debug)]
pub struct IndexedArray<T>(Vec<Option<Entry<T>>>);

impl<T> Default for IndexedArray<T> {
    fn default() -> Self {
        Self(Vec::<Option<Entry<T>>>::new())
    }
}

impl<T> IndexedArray<T> {
    pub fn set(&mut self, index: &VersionedIndex, value: T) {
        let i = index.index;

        if i >= self.0.len() {
            self.0.resize_with(i + 4, || None);
        }

        self.0[i] = Some(Entry {
            version: index.version,
            value
        });
    }

    pub fn get(&self, index: &VersionedIndex) -> Option<&T> {
        match self.0.get(index.index) {
            Some(Some(entry)) => {
                if entry.version == index.version {
                    Some(&entry.value)
                } else { None }
            },
            _ => None
        }
    }

    pub fn get_mut(&mut self, index: &VersionedIndex) -> Option<&mut T> {
        match self.0.get_mut(index.index) {
            None => None,
            Some(None) => None,
            Some(Some(entry)) => {
                if entry.version == index.version {
                    Some(&mut entry.value)
                } else { None }
            }
        }
    }

    /// TODO: write test
    pub fn get_entities(
        &'static self,
        allocator: &VersionedIndexAllocator
    ) -> Vec<IndexedEntry<T>> {
        self.0.iter()
            .enumerate()
            .filter_map(|(i, wrapped)| match wrapped {
                Some(entry) => {
                    let index = VersionedIndex {
                        index: i,
                        version: entry.version
                    };

                    match allocator.validate(&index) {
                        true => Some(IndexedEntry {
                            index,
                            entry: &entry.value
                        }),
                        false => None
                    }
                },
                None => None
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct IndexedEntry<T: 'static> {
    pub index: VersionedIndex,
    pub entry: &'static T
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct Entity(String);

    type EntityMap<T> = IndexedArray<T>;

    #[test]
    fn indexed_array_getting_setting_removing() {
        let mut allocator = VersionedIndexAllocator::default();
        let mut entities = EntityMap::<Entity>::default();

        let player_id = allocator.allocate();
        let npc_id = allocator.allocate();
        let enemy_id = allocator.allocate();

        entities.set(&player_id, Entity("player".to_string()));
        entities.set(&npc_id, Entity("npc".to_string()));
        entities.set(&enemy_id, Entity("enemy".to_string()));

        assert_eq!(allocator.length(), 3);

        let mut bullets = Vec::<VersionedIndex>::new();
        for _ in 0..3 {
            let bullet = allocator.allocate();

            entities.set(&bullet, Entity("bullet".to_string()));

            bullets.push(bullet);
        }

        assert_eq!(allocator.length(), 6);
        assert_eq!(entities.get(&player_id), Some(&Entity("player".to_string())));
        assert_eq!(entities.get(&enemy_id), Some(&Entity("enemy".to_string())));
        assert_eq!(entities.get(&npc_id), Some(&Entity("npc".to_string())));

        // npc_id is no longer valid after this call because the remove function destroys it
        allocator.deallocate(npc_id);

        assert_eq!(allocator.length(), 5);

        for bullet in bullets {
            allocator.deallocate(bullet);
        }

        assert_eq!(allocator.length(), 2);
        assert_eq!(entities.get(&player_id), Some(&Entity("player".to_string())));
        assert_eq!(entities.get(&enemy_id), Some(&Entity("enemy".to_string())));
    }

    #[test]
    fn indexed_array_versioning() {
        let mut allocator = VersionedIndexAllocator::default();
        let mut entities = EntityMap::<Entity>::default();

        let player_id = allocator.allocate();
        let npc_id = allocator.allocate();
        let enemy_id = allocator.allocate();

        entities.set(&player_id, Entity("player".to_string()));
        entities.set(&npc_id, Entity("npc".to_string()));
        entities.set(&enemy_id, Entity("enemy".to_string()));

        assert_eq!(
            entities.get(&VersionedIndex { index: 1, version: 0 }),
            Some(&Entity("npc".to_string()))
        );

        allocator.deallocate(npc_id);

        // used to hold npc
        assert!(!allocator.is_allocated(&VersionedIndex { index: 1, version: 0 }));

        let npc_id = allocator.allocate();
        entities.set(&npc_id, Entity("npc".to_string()));

        assert_eq!(
            entities.get(&VersionedIndex { index: 1, version: 1 }),
            Some(&Entity("npc".to_string()))
        );

        assert_eq!(
            entities.get(&VersionedIndex { index: 1, version: 0 }),
            None
        );

        // version 1 is allocated while version 0 is not
        assert!(!allocator.is_allocated(&VersionedIndex { index: 1, version: 0 }));
        assert!(allocator.is_allocated(&VersionedIndex { index: 1, version: 1 }));
    }
}
