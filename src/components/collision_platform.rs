use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Clone, Component, Debug)]
#[storage(DenseVecStorage)]
pub struct CollisionPlatform {
    pub height: f32,
    pub width: f32,
    pub x: f32,
    pub y: f32,
}

impl Default for CollisionPlatform {
    fn default() -> Self {
        CollisionPlatform {
            height: 0.,
            width: 0.,
            x: 0.,
            y: 0.,
        }
    }
}

impl CollisionPlatform {
    pub fn new(width: f32, height: f32, x: f32, y: f32) -> CollisionPlatform {
        CollisionPlatform {
            height,
            width,
            x,
            y,
        }
    }
}
