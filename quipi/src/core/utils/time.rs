use std::time::{
    Instant,
    SystemTime,
    SystemTimeError
};

#[derive(Debug)]
pub struct Timer {
    timer: Instant,
    last_tick: u128,
}

impl Timer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let timer = Instant::now();

        Ok(Self {
            timer,
            last_tick: timer.elapsed().as_millis()
        })
    }

    pub fn ticks(&self) -> u128 {
        self.timer.elapsed().as_millis()
    }

    pub fn delta(&mut self) -> f32 {
        let ticks = self.ticks();
        let delta = ticks - self.last_tick;

        self.last_tick = ticks;

        delta as f32
    }
}

pub fn now_milis() -> Result<u128, SystemTimeError> {
    Ok(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis())
}
pub fn now_secs() -> Result<u64, SystemTimeError> {
    Ok(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs())
}
