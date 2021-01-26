use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum PlayerState {
    Dying,
    Idling,
    Jumping,
    Running,
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
    pub has_jumped: bool,
    pub is_jumping: bool,
    pub max_ground_speed: f32,
    pub max_air_speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            state: PlayerState::Idling,
            is_jumping: false,
            has_jumped: false,
            max_ground_speed: 2.,
            max_air_speed: 5.,
        }
    }
}
