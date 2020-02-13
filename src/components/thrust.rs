use amethyst::{
    ecs::{DenseVecStorage, Component},
};

#[derive(Debug)]
pub struct Thrust {
    pub show: bool,
}

impl Component for Thrust {
    type Storage = DenseVecStorage<Self>;
}