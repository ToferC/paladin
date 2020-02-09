use amethyst::ecs::{Component, DenseVecStorage, Entity};

// EnemyAi is a state machine
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum EnemyAi {
    CloseDistance,
    Escape,
    MaintainDistance,
    Attack,
    Evade,
}

impl Default for EnemyAi {
    fn default() -> Self {
        EnemyAi::CloseDistance
    }
}

