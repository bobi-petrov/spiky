use amethyst::{
    core::math::Vector2,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Collider, Direction, Player, PlayerState, Motion};

pub struct KinematicsSystem;

impl<'s> System<'s> for KinematicsSystem {
    type SystemData = (WriteStorage<'s, Collider>, ReadStorage<'s, Motion>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut colliders, motions) = data;

        for (collider, motion) in (&mut colliders, &motions).join() {
            let bbox = &mut collider.bounding_box;
            bbox.old_position = bbox.position;
            bbox.position.x += motion.velocity.x;
            bbox.position.y += motion.velocity.y;

            let hbox = &mut collider.hit_box;
            hbox.old_position = hbox.position;
            collider.set_hit_box_position(motion.velocity);
        }
    }
}

pub struct PlayerKinematicsSystem;

impl<'s> System<'s> for PlayerKinematicsSystem {
    type SystemData = (
        WriteStorage<'s, Collider>,
        ReadStorage<'s, Direction>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Motion>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut colliders, dirs, players, mut motions) = data;

        for (collider, dir, player, motion) in
            (&mut colliders, &dirs, &players, &mut motions).join()
        {
            let mut acceleration = Vector2::new(0., 0.);
            match player.state {
                PlayerState::Idling => {
                    let acceleration_x = if motion.velocity.x != 0. { -0.6 } else { 0. };
                    acceleration = Vector2::new(acceleration_x, -0.6);
                }
                PlayerState::Running => {
                    acceleration = Vector2::new(0.6, -0.6);
                }
                PlayerState::Jumping => {
                    if collider.on_ground {
                        motion.velocity.y = 14.;
                        collider.on_ground = false;
                    }
                    let acceleration_x = if motion.velocity.x != 0. { -0.06 } else { 0. };
                    acceleration = Vector2::new(acceleration_x, -0.6);
                }
                PlayerState::Dying => {
                    if collider.on_ground {
                        motion.velocity.x = 0.;
                        motion.velocity.y = 8.;
                        collider.on_ground = false;
                    }
                    acceleration = Vector2::new(0., -0.6);
                }
            }
            motion.update_velocity(acceleration, dir, 0., player.max_ground_speed);
        }
    }
}
