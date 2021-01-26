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
    components::{
        Animation, AnimationId, AnimationPrefabData, Boundary, Collidee, Collider, Direction,
        Directions, Motion, Player,
    },
    resources::Context,
};

pub fn load_player(world: &mut World, prefab: Handle<Prefab<AnimationPrefabData>>, ctx: &Context) {
    let scale = ctx.scale;
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(scale, scale, scale));
    transform.set_translation_x(100.);
    transform.set_translation_y(176.);

    let mut collider = Collider::new(32. * scale, 36. * scale);
    let bbox = &mut collider.bounding_box;
    bbox.position = Vector2::new(100., 176.);
    bbox.old_position = bbox.position;

    let motion = Motion::new();
    collider.set_hit_box_position(motion.velocity);

    world
        .create_entity()
        .with(Player::new())
        .named("Player")
        .with(collider)
        .with(Collidee::default())
        .with(Boundary::new(ctx.x_correction, ctx.map_width, 526., 0.))
        .with(transform)
        .with(motion)
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
        .with(Direction::new(
            Directions::Right,
            Directions::Neutral,
            Directions::Right,
            Directions::Neutral,
        ))
        .with(Transparent) // Necessary for ordered layering
        .build();
}
