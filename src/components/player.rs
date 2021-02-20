use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum PlayerState {
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
}

impl Player {
    pub fn new() -> Self {
        Player {
            state: PlayerState::Falling,
        }
    }
}
