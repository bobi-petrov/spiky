use crate::{
    components::{Player, PlayerState},
    resources::Context,
};
use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadExpect, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use std::f32::consts::PI;
pub struct PlayerInputSystem;

impl<'s> System<'s> for PlayerInputSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, Context>,
    );

    fn run(&mut self, (mut players, input, mut transforms, ctx): Self::SystemData) {
        for (player, transform) in (&mut players, &mut transforms).join() {
            let run_input = input.axis_value("run").expect("Run action exists");
            let jump_input = input.action_is_down("jump").expect("Jump action exists");

            player.state = if jump_input
                && player.state != PlayerState::Falling
                && player.state != PlayerState::Jumping
            {
                let scaled_y_amount = ctx.scale * 16. + 35.;
                transform.prepend_translation_y(scaled_y_amount);
                PlayerState::Jumping
            } else if run_input != 0. {
                let scaled_amount = ctx.scale * 2. * run_input;
                // rotate player
                transform.set_rotation_y_axis(0.);
                if run_input < 0. {
                    transform.set_rotation_y_axis(PI);
                }

                transform.prepend_translation_x(scaled_amount);
                PlayerState::Running
            } else {
                PlayerState::Falling
            }
        }
    }
}
