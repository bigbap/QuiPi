use crate::QPResult;
use std::time::{Instant, SystemTime};

#[derive(Debug, PartialEq, Clone)]
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

    pub fn delta(&mut self) -> u128 {
        let ticks = self.ticks();
        let delta = ticks - self.last_tick;

        self.last_tick = ticks;

        delta
    }

    pub fn elapsed(&self) -> u128 {
        self.ticks() - self.first_tick
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Interval {
    timer: Timer,
    interval: u128,
    last_tick: u128,
}

impl Interval {
    /*
     * interval is in seconds
     */
    pub fn new(interval: u128) -> Self {
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

#[derive(Debug, PartialEq, Clone)]
pub struct Countdown {
    timer: Timer,
    pub countdown: u128,
    start: u128,
}

impl Countdown {
    pub fn new(countdown: u128) -> Self {
        let timer = Timer::new();
        let start = timer.elapsed();

        Self {
            timer,
            countdown,
            start,
        }
    }

    pub fn check(&mut self) -> u128 {
        let delta = self.timer.elapsed() - self.start;

        self.countdown - delta.min(self.countdown)
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
