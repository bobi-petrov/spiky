use amethyst::{
    ecs::{Join, Read, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::{Collider, Direction, Directions, Player, PlayerState};

pub struct PlayerInputSystem;

impl<'s> System<'s> for PlayerInputSystem {
    type SystemData = (
        WriteStorage<'s, Direction>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Collider>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut dir, mut players, mut colliders, input) = data;

        for (dir, player, collider) in (&mut dir, &mut players, &mut colliders).join() {
            let run_input = input.axis_value("run").expect("Run action exists");
            let jump_input = input.action_is_down("jump").expect("Jump action exists");

            // TODO: check simultaneous button press
            player.state = if !collider.is_collidable {
                PlayerState::Dying
            } else if jump_input || !collider.on_ground {
                PlayerState::Jumping
            } else if run_input > 0. {
                dir.x = Directions::Right;
                PlayerState::Running
            } else if run_input < 0. {
                dir.x = Directions::Left;
                PlayerState::Running
            } else {
                PlayerState::Idling
            }
        }
    }
}
