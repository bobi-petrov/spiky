use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System},
};

use crate::components::Player;

pub struct PlayerDeathSystem;

impl<'s> System<'s> for PlayerDeathSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, players, transforms) = data;
        for (entity, _, transform) in (&entities, &players, &transforms).join() {
            if transform.translation().y < -999. {
                let _ = entities.delete(entity);
            }
        }
    }
}
