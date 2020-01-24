use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{transform::Transform, math},
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const ARENA_HEIGHT: f32 = 900.0;
pub const ARENA_WIDTH: f32 = 1200.0;

pub const SHIP_SCALING: f32 = 0.20;

pub const LASER_VELOCITY_X: f32 = 75.0;
pub const LASER_VELOCITY_Y: f32 = 50.0;
pub const LASER_RADIUS: f32 = 1.0;
pub const LASER_MAX_LIFE: f32 = 8.0;

const SHIP_HEIGHT: f32 = 32.0;
const SHIP_WIDTH: f32 = 32.0;

#[derive(Default)]
pub struct Paladin {
    laser_life_timer: Option<f32>,
}

impl SimpleState for Paladin {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        // `spritesheet` is the layout of the sprites on the image;
        // `texture` is the pixel data.

        world.register::<Laser>();


        let ship_sheet_handle = load_sprite_sheet(world, "texture/ship_spritesheet");
        let bullet_sheet_handle = load_sprite_sheet(world, "texture/bullet");

        LaserRes::initialise(world);

        initialise_ships(world, ship_sheet_handle);
        shoot_laser(world, bullet_sheet_handle);
        initialise_camera(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {

        let world = &mut data.world;

        world.maintain();

        Trans::None
    }
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Light,
    Dark,
}

pub struct Ship {
    pub side: Side,
    pub width: f32,
    pub height: f32,
    pub agility: f32,
    pub acceleration: f32,
    pub velocity: [f32; 2],
    pub laser_velocity: f32,
}

impl Ship {
    fn new(side: Side) -> Ship {
        Ship {
            side,
            width: SHIP_WIDTH,
            height: SHIP_HEIGHT,
            agility: 0.05,
            acceleration: 0.75,
            velocity: [0.0, 0.0],
            laser_velocity: 10.0,
        }
    }
}

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}

pub struct Laser {
    pub timer: f32,
}

impl Component for Laser {
    type Storage = DenseVecStorage<Self>;
}

impl Laser {
    pub fn new() -> Laser {
        Laser {
            timer: 6.0,
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

#[derive(Debug)]
pub struct Physical {
    pub velocity: math::Vector2<f32>,
    pub max_velocity: f32,
    pub rotation: f32,
    pub radius: f32,
}

impl Component for Physical {
    type Storage = DenseVecStorage<Self>;
}

impl Physical {
    pub fn new(radius: f32) -> Physical {
        Physical {
            velocity: math::Vector2::new(0.0, 0.0),
            max_velocity: 110.0,
            rotation: 0.0,
            radius: radius
        }
    }
}

fn shoot_laser(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    let physical = Physical::new(4.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(physical)
        .with(Laser {
            timer: LASER_MAX_LIFE,
        })
        .with(local_transform)
        .build();
}

fn load_sprite_sheet(world: &mut World, path: &str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("{}.png", path),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("{}.ron", path),
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

/// Initialise the camera.
fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom light.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

/// Initialises one ship on the light, and one ship on the dark.
fn initialise_ships(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut light_transform = Transform::default();
    let mut dark_transform = Transform::default();

    // rescale ships
    light_transform.set_scale(math::Vector3::new(SHIP_SCALING, SHIP_SCALING, SHIP_SCALING));
    dark_transform.set_scale(math::Vector3::new(SHIP_SCALING, SHIP_SCALING, SHIP_SCALING));

    // rotate ships
    light_transform.rotate_2d(1.60);
    dark_transform.rotate_2d(-1.60);

    // Correctly position the ships.
    let y = ARENA_HEIGHT / 2.0;
    light_transform.set_translation_xyz(SHIP_WIDTH * 3.0, y, 0.0);
    dark_transform.set_translation_xyz(ARENA_WIDTH - SHIP_WIDTH * 3.0, y, 0.0);

    // Assign the sprites for the ships
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0, // ship is the first sprite in the sprite_sheet
    };

    // Create a light ship entity.
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Ship::new(Side::Light))
        .with(light_transform)
        .with(Physical::new(24.0))
        .build();

    // Create dark ship entity.
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Ship::new(Side::Dark))
        .with(dark_transform)
        .with(Physical::new(24.0))
        .build();
}
