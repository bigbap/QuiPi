use crate::QPResult;
use std::time::{Instant, SystemTime};

#[derive(Debug)]
pub struct Timer {
    timer: Instant,
    last_tick: u128,
    first_tick: u128,
}

impl Timer {
    pub fn new() -> Self {
        let timer = Instant::now();
        let last_tick = timer.elapsed().as_millis();
        let first_tick = last_tick;

        Self {
            timer,
            last_tick,
            first_tick,
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

    pub fn elapsed(&self) -> f32 {
        let tick = self.ticks();

        (tick - self.first_tick) as f32 / 1000.0
    }
}

pub struct Interval {
    timer: Timer,
    interval: f32,
    last_tick: f32,
}

impl Interval {
    /*
     * interval is in seconds
     */
    pub fn new(interval: f32) -> Self {
        let timer = Timer::new();
        let last_tick = timer.elapsed();

        Self {
            timer,
            last_tick,
            interval,
        }
    }

    pub fn check(&mut self) -> bool {
        let delta = self.timer.elapsed() - self.last_tick;

        if delta >= self.interval {
            self.last_tick += delta - (delta % self.interval);

            return true;
        }

        false
    }
}

pub struct Countdown {
    timer: Timer,
    pub countdown: f32,
    start: f32,
}

impl Countdown {
    pub fn new(countdown: f32) -> Self {
        let timer = Timer::new();
        let start = timer.elapsed();

        Self {
            timer,
            countdown,
            start,
        }
    }

    pub fn check(&mut self) -> f32 {
        let delta = self.timer.elapsed() - self.start;

        (self.countdown - delta).max(0.0)
    }
}

pub fn now_milis() -> QPResult<u128> {
    Ok(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_millis())
}
pub fn now_secs() -> QPResult<u64> {
    Ok(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs())
}
