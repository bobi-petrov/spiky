use amethyst::{
    core::{math::Vector2, Named},
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Boundary, Collidee, CollideeDetails, Collider, Motion};

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Collider>,
        WriteStorage<'s, Collidee>,
        ReadStorage<'s, Boundary>,
        ReadStorage<'s, Motion>,
        ReadStorage<'s, Named>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, colliders, mut collidees, boundaries, motions, names) = data;

        for (entity_a, collider_a, collidee, boundary, motion_a) in
            (&entities, &colliders, &mut collidees, &boundaries, &motions).join()
        {
            let velocity_a = motion_a.velocity;
            let bbox_a = &collider_a.bounding_box;
            let position_a_x = bbox_a.position.x;
            let half_size_a_x = bbox_a.half_size.x;
            let correction;

            if velocity_a.x != 0. || velocity_a.y != 0. && collider_a.is_collidable {
                for (entity_b, collider_b, motion_b, name_b) in
                    (&entities, &colliders, &motions, &names).join()
                {
                    let velocity_b = motion_b.velocity;
                    let use_hit_box =
                        (velocity_a.x * velocity_b.x != 0.) || (velocity_a.y * velocity_b.y != 0.);
                    if entity_a != entity_b
                        && collider_a.is_overlapping_with(collider_b, use_hit_box)
                    {
                        collidee.set_collidee_details(
                            name_b.name.to_string(),
                            collider_a,
                            collider_b,
                            velocity_a,
                            velocity_b,
                            use_hit_box,
                        );
                    }
                }
            }

            correction = if (position_a_x - half_size_a_x) <= boundary.left {
                (position_a_x - half_size_a_x) - boundary.left
            } else if (position_a_x + half_size_a_x) >= boundary.right {
                (position_a_x + half_size_a_x) - boundary.right
            } else {
                0.
            };

            if correction != 0. {
                collidee.horizontal = Some(CollideeDetails {
                    name: String::from("Boundary"),
                    position: Vector2::new(0., 0.),
                    half_size: Vector2::new(0., 0.),
                    correction,
                });
            }
        }
    }
}
