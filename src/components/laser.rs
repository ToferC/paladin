use amethyst::ecs::{Component, NullStorage, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender};

use super::ship::Side;

use crate::resources::load_sprite_sheet;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct LaserImpact;

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

pub struct LaserRes {
    pub sprite_render: SpriteRender
}

impl Component for LaserRes {
    type Storage = DenseVecStorage<Self>;
}

impl LaserRes {
    pub fn initialise(world: &mut World) {
        let sprite_sheet_handle = load_sprite_sheet(world, "texture/bullet");

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 0,
        };

        world.insert(LaserRes { sprite_render });
    }

    pub fn sprite_render(&self) -> SpriteRender {
        self.sprite_render.clone()
    }
}