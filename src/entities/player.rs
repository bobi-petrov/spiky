use amethyst::{
    assets::{Handle, Prefab},
    core::{
        math::{Vector2, Vector3},
        Transform, WithNamed,
    },
    ecs::prelude::World,
    prelude::{Builder, WorldExt},
    renderer::transparent::Transparent,
};

use crate::{
    components::{Animation, AnimationId, AnimationPrefabData, Player},
    resources::Context,
};

pub fn load_player(world: &mut World, prefab: Handle<Prefab<AnimationPrefabData>>, ctx: &Context) {
    let scale = ctx.scale;
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(scale, scale, scale));
    transform.set_translation_x(75.);
    transform.set_translation_y(200.);

    world
        .create_entity()
        .with(Player::new())
        .named("Player")
        .with(transform)
        .with(Animation::new(
            AnimationId::Idle,
            vec![
                AnimationId::Die,
                AnimationId::Idle,
                AnimationId::Jump,
                AnimationId::Move,
            ],
        ))
        .with(prefab)
        .with(Transparent) // Necessary for ordered layering
        .build();
}
