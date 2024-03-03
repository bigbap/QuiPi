use rand::{Rng, SeedableRng};

pub struct Random {
    rng: rand_chacha::ChaCha8Rng,
}

impl Random {
    pub fn from_seed(seed: u64) -> Self {
        let rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

        Self { rng }
    }

    pub fn random(&mut self) -> f32 {
        self.rng.gen::<f32>()
    }

    pub fn range(&mut self, start: i32, end: i32) -> i32 {
        self.rng.gen_range(start..end)
    }

    pub fn binary(&mut self, bias: f32) -> bool {
        self.random() < bias
    }
}
