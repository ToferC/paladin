use amethyst::{
    core::{transform::Transform, Parent, Hidden, math},
    ecs::prelude::{Component, DenseVecStorage, LazyUpdate},
    prelude::*,
    renderer::{SpriteRender, Transparent,
        resources::Tint,
        palette::Srgba,
    },
};

use super::physical::Physical;
use super::combat::{Combat, LaserType};
use super::enemy::EnemyAi;
use super::thrust::Thrust;
use crate::resources::{SpriteSheetList, AssetType};

use crate::paladin::{ARENA_HEIGHT, ARENA_WIDTH};

pub const SHIP_SCALING: f32 = 0.20;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Side {
    Light,
    Dark,
}

#[derive(Debug)]
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
    
    // Get SpriteSheetList
    let sprite_sheet_list = {
        world.try_fetch::<SpriteSheetList>().expect("Unable to fetch SpriteSheetList")
    };

    let light_sprite_sheet_handle = sprite_sheet_list.get(AssetType::LightShip).unwrap();
    let dark_sprite_sheet_handle = sprite_sheet_list.get(AssetType::DarkShip).unwrap();

    let mut light_transform = Transform::default();
    let mut dark_transform = Transform::default();

    // rescale ships
    light_transform.set_scale(math::Vector3::new(SHIP_SCALING, SHIP_SCALING, SHIP_SCALING));
    dark_transform.set_scale(math::Vector3::new(SHIP_SCALING, SHIP_SCALING, SHIP_SCALING));

    // rotate ships
    light_transform.rotate_2d(1.60);
    dark_transform.rotate_2d(-1.60);

    let light_phys = Physical::new(43.0, 100.0, 1.25, 0.05);
    let dark_phys = Physical::new(41.0, 100.0, 1.35, 0.05);


    // Correctly position the ships.
    let y = ARENA_HEIGHT / 2.0;
    light_transform.set_translation_xyz(&light_phys.radius * 4.0, y, 0.0);
    dark_transform.set_translation_xyz(ARENA_WIDTH - &dark_phys.radius * 4.0, y, 0.0);

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

    // Load resources for adding thrust
    let lazy = world.try_fetch::<LazyUpdate>().expect("Unable to load LazyUpdate");


    // Get Sprite sheet handle
    let thrust_sprite_sheet_handle = sprite_sheet_list.get(AssetType::Thrust).unwrap();

    // Construct sprite render for thruster
    let thrust_sprite_render = SpriteRender {
        sprite_sheet: thrust_sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    // Create a light ship entity.
    let light_ship = world.entities().create();
    lazy.insert(light_ship, Ship::new(Side::Light));
    lazy.insert(light_ship, light_sprite_render.clone());
    lazy.insert(light_ship, light_transform);
    lazy.insert(light_ship, light_phys.clone());
    lazy.insert(light_ship, Combat::new(150, 6, 20, 6.0, 10.0, LaserType::Single, 0.2, 25, 30.0, 6.0, 5.0));

    // Create thrust entity for light ship
    let light_thrust = world.entities().create();
    
    lazy.insert(
        light_thrust,
        Thrust {
        show: false,
    });
    lazy.insert(
        light_thrust,
        thrust_sprite_render.clone(),
    );
    lazy.insert(light_thrust, Parent::new(light_ship));

    let mut light_thrust_transform = Transform::from(math::Vector3::<f32>::new(
        0., -200., 0.
    ));
    light_thrust_transform.rotate_2d(-1.6);

    lazy.insert(light_thrust, light_thrust_transform);
    lazy.insert(light_thrust, Transparent);
    lazy.insert(light_thrust, Hidden);

    
    // Create a dark ship entity.
    let dark_ship = world.entities().create();
    lazy.insert(dark_ship, Ship::new(Side::Dark));
    lazy.insert(dark_ship, dark_sprite_render.clone());
    lazy.insert(dark_ship, dark_transform);
    lazy.insert(dark_ship, dark_phys.clone());
    lazy.insert(dark_ship, Combat::new(130, 5, 15, 4.0, 10.0, LaserType::Dual, 0.3, 25, 30.0, 6.0, 5.0));

    // Create thrust entity for dark ship
    let dark_thrust = world.entities().create();

    // Set tint for dark thrust to yellow
    let tint = Tint(Srgba::new(0.7, 0.7, 0., 1.));
    
    lazy.insert(
        dark_thrust,
        Thrust {
        show: false,
    });
    lazy.insert(
        dark_thrust,
        thrust_sprite_render.clone(),
    );
    lazy.insert(dark_thrust, Parent::new(dark_ship));
    let mut dark_thrust_transform = Transform::from(math::Vector3::<f32>::new(
        0., -240., 0.
    ));
    dark_thrust_transform.rotate_2d(-1.6);

    lazy.insert(dark_thrust, dark_thrust_transform);
    lazy.insert(dark_thrust, Transparent);
    lazy.insert(dark_thrust, Hidden);
    lazy.insert(dark_thrust, tint);
}