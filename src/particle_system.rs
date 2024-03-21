use crate::prelude::*;

use self::qp_common::components::CColor;

pub struct ParticleSystem;

impl Plugin for ParticleSystem {
    fn build(&self, app: &mut App) -> crate::QPResult<()> {
        app.add_system(Update, |storage: ResMut<StorageManager>| {
            let Some(storage) = storage else {
                return;
            };

            let Some(entities) = storage.get_mut(StorageId::Entities) else {
                return;
            };

            let particles = match entities.get_component_list_mut::<CParticle>() {
                Some(p) => p,
                _ => return,
            };

            let mut to_despawn: Vec<Index> = vec![];
            let mut to_change: Vec<(Index, u128, u128)> = vec![];
            for particle in particles.iter_mut() {
                if particle.is_none() {
                    continue;
                }

                let (entity, particle) = particle.unwrap();
                let time_left = particle.countdown.check();
                let total_time = particle.countdown.countdown;

                if time_left == 0 {
                    to_despawn.push(entity);

                    continue;
                }

                to_change.push((entity, time_left, total_time));
            }

            for (entity, time_left, total_time) in to_change.iter() {
                if let Some(color) = entities.get_mut::<CColor>(&entity) {
                    color.3 = *time_left as f32 / *total_time as f32;
                }
            }
            for entity in to_despawn.iter() {
                entities.despwan(*entity);
            }
        });

        Ok(())
    }
}

#[derive(Debug, Component, PartialEq, Clone)]
pub struct CParticle {
    pub countdown: Countdown,
}
