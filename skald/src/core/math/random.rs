use rand::{
    SeedableRng,
    Rng
};

pub struct Random {
    rng: rand_chacha::ChaCha8Rng
}

impl Random {
    pub fn from_seed(seed: u64) -> Self {
        let rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

        Self {
            rng
        }
    }

    pub fn random(&mut self) -> f32 {
        self.rng.gen::<f32>()
    }

    pub fn range(&mut self, start: u32, end: u32) -> u32 {
        self.rng.gen_range(start..end)
    }
}
