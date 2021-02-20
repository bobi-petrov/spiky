use crate::{
    components::{CollisionPlatform, Player, PlayerState, Subject},
    resources::Context,
};
use amethyst::{
    core::Transform,
    ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
};

pub struct TransformationSystem;

impl<'s> System<'s> for TransformationSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, CollisionPlatform>,
        WriteStorage<'s, Player>,
        ReadExpect<'s, Context>,
    );

    fn run(&mut self, (mut transforms, mut platforms, mut players, ctx): Self::SystemData) {
        for (player, player_transform) in (&mut players, &mut transforms).join() {
            let mut grounded = false;

            let player_x = player_transform.translation().x + (12. * 0.25);
            let player_y = player_transform.translation().y + (16. * 0.25);
            for platform in (&mut platforms).join() {
                // has died -> restart
                if player_y <= 70. {
                    player_transform.set_translation_x(50.);
                    player_transform.set_translation_y(200.);
                }

                // Collision
                if player_x > platform.x
                    && player_x < platform.x + platform.width + 4.
                    && player_y > platform.y
                    && (player_y < platform.y + platform.height)
                {
                    if platform.is_spike {
                        platform.collidee = Some(true);
                        player_transform.set_translation_x(50.);
                        player_transform.set_translation_y(200.);
                    }
                    if player_y + 2. >= platform.y + platform.height {
                        // from top
                        grounded = true;
                    }
                }
            }
            if grounded && player.state != PlayerState::Running {
                player.state = PlayerState::Idling;
            }
            if !grounded && player.state != PlayerState::Jumping {
                // falling
                player.state = PlayerState::Falling;
                let scaled_y_amount = -1. * ctx.scale * 1.5;
                player_transform.prepend_translation_y(scaled_y_amount);
            }
        }
    }
}

pub struct CameraTransformationSystem;

impl<'s> System<'s> for CameraTransformationSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Subject>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, Context>,
    );

    fn run(&mut self, (players, subject_tags, mut transforms, ctx): Self::SystemData) {
        let mut player_x = 0.;
        let map_width = ctx.map_width;
        let background_width = ctx.bg_width;

        for (_player, transform) in (&players, &transforms).join() {
            player_x = transform.translation().x;
        }

        for (_subject_tag, transform) in (&subject_tags, &mut transforms).join() {
            if player_x == 50. {
                // died
                transform.set_translation_xyz(576., 192. * 1.5, 0.);
            }
            if player_x >= background_width && player_x <= map_width - background_width {
                transform.set_translation_x(player_x);
            }
        }
    }
}
