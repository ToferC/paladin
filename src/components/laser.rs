use amethyst::ecs::{Component, NullStorage, DenseVecStorage};

use super::ship::Side;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct LaserImpact;

#[derive(Debug, Clone, Copy)]
pub struct Laser {
    pub timer: f32,
    pub damage: i32,
    pub side: Side,
}

impl Component for Laser {
    type Storage = DenseVecStorage<Self>;
}

impl Laser {
    pub fn new(timer: f32, damage: i32, side: Side) -> Laser {
        Laser {
            timer,
            damage,
            side,
        }
    }
}