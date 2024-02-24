use std::time::{
    Instant,
    SystemTime
};
use crate::QPResult;

#[derive(Debug)]
pub struct Timer {
    timer: Instant,
    last_tick: u128,
}

impl Timer {
    pub fn new() -> Self {
        let timer = Instant::now();

        Self {
            timer,
            last_tick: timer.elapsed().as_millis()
        }
    }

    pub fn ticks(&self) -> u128 {
        self.timer.elapsed().as_millis()
    }

    pub fn delta(&mut self) -> f32 {
        let ticks = self.ticks();
        let delta = ticks - self.last_tick;

        self.last_tick = ticks;

        delta as f32 / 1000.0
    }
}

pub fn now_milis() -> QPResult<u128> {
    Ok(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis())
}
pub fn now_secs() -> QPResult<u64> {
    Ok(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs())
}
