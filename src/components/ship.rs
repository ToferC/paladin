use amethyst::{
    core::{transform::Transform, math},
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender,
    },
};

use crate::resources::load_sprite_sheet;

use super::physical::Physical;
use super::combat::Combat;

use crate::paladin::{ARENA_HEIGHT, ARENA_WIDTH};

pub const SHIP_SCALING: f32 = 0.20;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Side {
    Light,
    Dark,
}

pub struct Ship {
    pub side: Side,
    pub thrust_timer: f32,
}

impl Ship {
    pub fn new(side: Side) -> Ship {
        Ship {
            side,
            thrust_timer: 0.0,
        }
    }
}

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}

/// Initialises one ship on the light, and one ship on the dark.
pub fn initialise_ships(world: &mut World) {

    let light_sprite_sheet_handle = load_sprite_sheet(world, "texture/ship_spritesheet");
    let dark_sprite_sheet_handle = load_sprite_sheet(world, "texture/dark_ship_spritesheet");


    let mut light_transform = Transform::default();
    let mut dark_transform = Transform::default();

    // rescale ships
    light_transform.set_scale(math::Vector3::new(SHIP_SCALING, SHIP_SCALING, SHIP_SCALING));
    dark_transform.set_scale(math::Vector3::new(SHIP_SCALING, SHIP_SCALING, SHIP_SCALING));

    // rotate ships
    light_transform.rotate_2d(1.60);
    dark_transform.rotate_2d(-1.60);

    let phys = Physical::new(43.0, 100.0, 1.25, 0.05);

    // Correctly position the ships.
    let y = ARENA_HEIGHT / 2.0;
    light_transform.set_translation_xyz(&phys.radius * 4.0, y, 0.0);
    dark_transform.set_translation_xyz(ARENA_WIDTH - &phys.radius * 4.0, y, 0.0);

    // Assign the sprites for the light ship
    let light_sprite_render = SpriteRender {
        sprite_sheet: light_sprite_sheet_handle.clone(),
        sprite_number: 0, // ship is the first sprite in the sprite_sheet
    };

    // Assign the sprites for the dark ship
    let dark_sprite_render = SpriteRender {
        sprite_sheet: dark_sprite_sheet_handle.clone(),
        sprite_number: 0, // ship is the first sprite in the sprite_sheet
    };

    // Create a light ship entity.
    world
        .create_entity()
        .with(light_sprite_render.clone())
        .with(Ship::new(Side::Light))
        .with(light_transform)
        .with(phys.clone())
        .with(Combat::new(150, 5, 10, 6.0, 10.0, 25, 30.0, 6.0, 5.0))
        .build();

    // Create dark ship entity.
    world
        .create_entity()
        .with(dark_sprite_render.clone())
        .with(Ship::new(Side::Dark))
        .with(dark_transform)
        .with(phys.clone())
        .with(Combat::new(150, 5, 10, 6.0, 10.0, 25, 30.0, 6.0, 5.0))
        .build();
}