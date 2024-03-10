use crate::{core::prelude::Timer, resources::Resource};

#[derive(Resource)]
pub struct Clock {
    timer: Timer,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            timer: Timer::new(),
        }
    }

    pub fn elapsed(&mut self) -> u128 {
        self.timer.elapsed()
    }
}
