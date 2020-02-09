use amethyst::core::math;
use amethyst::ecs::{DenseVecStorage, Component};

/// Physical represents the physics system in the game
#[derive(Debug, Clone, Copy)]
pub struct Physical {
    pub acceleration: f32,
    pub agility: f32,
    pub velocity: math::Vector2<f32>,
    pub max_velocity: f32,
    pub rotation: f32,
    pub radius: f32,
    pub mass: f32,
}

impl Component for Physical {
    type Storage = DenseVecStorage<Self>;
}

impl Physical {
    pub fn new(radius: f32, mass: f32, acceleration: f32, agility: f32) -> Physical {
        Physical {
            velocity: math::Vector2::new(0.0, 0.0),
            acceleration: acceleration,
            agility: agility,
            max_velocity: 5.0,
            rotation: 0.0,
            radius: radius,
            mass: mass,
        }
    }
}