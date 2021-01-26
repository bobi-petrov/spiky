use amethyst::{
    ecs::{Component, DenseVecStorage},
};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum PlayerState {
    Dying,
    Idling,
    Jumping,
    Running,
    Falling,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Idling
    }
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Player {
    pub state: PlayerState,
    pub max_ground_speed: f32,
    pub max_air_speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            state: PlayerState::Falling,
            max_ground_speed: 6.,
            max_air_speed: 12.,
        }
    }
}
