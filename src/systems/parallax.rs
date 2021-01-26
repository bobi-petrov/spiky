use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Collider, Player, Motion, Parallax};

#[derive(Default)]
pub struct ParallaxSystem;

impl<'s> System<'s> for ParallaxSystem {
    type SystemData = (
        ReadStorage<'s, Parallax>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Motion>,
        ReadStorage<'s, Collider>,
        WriteStorage<'s, Transform>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (parallaxes, players, motions, colliders, mut transforms) = data;
        let mut player_velocity_x = 0.;
        let mut player_moved = false;

        for (_, motion, collider) in (&players, &motions, &colliders).join() {
            player_velocity_x = motion.velocity.x;
            let bbox = &collider.bounding_box;
            player_moved = (bbox.position.x - bbox.old_position.x).abs() > std::f32::EPSILON;
        }

        for (_, transform) in (&parallaxes, &mut transforms).join() {
            if player_moved {
                transform.set_translation_x(
                    transform.translation().x
                        + player_velocity_x / (transform.translation().z.abs() * 4. / 10.),
                );
            }
        }
    }
}
