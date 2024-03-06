use qp_core::Interval;
use quipi::prelude::*;

pub struct ParticleSystem<P: Particle> {
    spawner: Box<dyn FnMut(&mut World) -> Option<P>>,
    particles: Vec<P>,
    interval: Interval,
    pub active: bool,
}

impl<P: Particle> ParticleSystem<P> {
    pub fn new(
        interval: f32,
        active: bool,
        spawner: impl FnMut(&mut World) -> Option<P> + 'static,
    ) -> Self {
        Self {
            spawner: Box::new(spawner),
            particles: vec![],
            interval: Interval::new(interval),
            active,
        }
    }

    pub fn update(&mut self, world: &mut World) -> Result<(), QPError> {
        for particle in self.particles.iter_mut() {
            particle.update(world);
        }

        if self.active && self.interval.check() {
            if let Some(particle) = (self.spawner)(world) {
                self.particles.push(particle);
            }
        }

        Ok(())
    }
}

pub trait Particle {
    fn update(&mut self, world: &mut World);
}
