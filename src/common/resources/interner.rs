use crate::resources::*;
use std::collections::HashMap;

#[derive(Debug, Resource)]
pub struct StringInterner {
    pub strings: HashMap<u64, String>,
}

impl StringInterner {
    pub fn new() -> Self {
        Self {
            strings: HashMap::new(),
        }
    }

    pub fn intern(&mut self, string: String) -> u64 {
        let hash = xx_hash(string.as_bytes(), 0xD2ABA3FA440449FB);

        if self.strings.get(&hash).is_none() {
            self.strings.insert(hash, string);
        }

        hash
    }

    pub fn get_string(&self, hash: u64) -> Option<String> {
        self.strings.get(&hash).cloned()
    }
}

fn xx_hash(input: &[u8], seed: u64) -> u64 {
    let mut hash = seed.wrapping_add(0xEC55E2EF86D31E87);
    let mut temp_hash: u64 = 0;

    // process each byte
    for &byte in input {
        temp_hash = temp_hash.wrapping_add(byte as u64);
        temp_hash = temp_hash.wrapping_mul(0xF6E716E11E254A03);
        temp_hash = temp_hash.rotate_left(31);
        temp_hash = temp_hash.wrapping_mul(0x9E6B71B56B368EA7);
        hash ^= temp_hash;
        hash = hash.rotate_left(27).wrapping_add(hash);
        hash = hash.wrapping_mul(5).wrapping_add(0xD14FFEC0);
    }

    // finalize
    hash ^= hash >> 33;
    hash = hash.wrapping_mul(0xB1F24A6E5B47623C);
    hash ^= hash >> 33;
    hash = hash.wrapping_mul(0x8235FB91FBCF147B);
    hash ^= hash >> 33;

    hash
}
