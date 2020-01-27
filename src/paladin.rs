use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{transform::Transform, math},
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::systems::audio::initialize_audio;

pub const ARENA_HEIGHT: f32 = 1024.0;
pub const ARENA_WIDTH: f32 = 1600.0;

pub const SHIP_SCALING: f32 = 0.20;

pub const LASER_RADIUS: f32 = 1.0;

const SHIP_HEIGHT: f32 = 60.8;
const SHIP_WIDTH: f32 = 73.2;

#[derive(Default)]
pub struct Paladin;

impl SimpleState for Paladin {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        // `spritesheet` is the layout of the sprites on the image;
        // `texture` is the pixel data.

        world.register::<Laser>();
        initialize_audio(world);


        let ship_sheet_handle = load_sprite_sheet(world, "texture/ship_spritesheet");
        //let force_field_sheet_handle = load_sprite_sheet(world, "texture/force_field");

        LaserRes::initialise(world);
        world.insert(RandomGen);

        initialize_scoreboard(world);
        initialize_ship_structures(world);

        initialise_ships(world, ship_sheet_handle);
        //initialize_force_field(world, force_field_sheet_handle);
        initialise_camera(world);

    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {

        let world = &mut data.world;

        world.maintain();

        Trans::None
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
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
    pub laser_velocity: f32,

    pub thrust_timer: f32,
}

impl Ship {
    fn new(side: Side) -> Ship {
        Ship {
            side,
            width: SHIP_WIDTH,
            height: SHIP_HEIGHT,
            agility: 0.05,
            acceleration: 0.75,
            laser_velocity: 10.0,

            thrust_timer: 0.0,
        }
    }
}

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}

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

/// Physical represents the physics system in the game
#[derive(Debug, Clone, Copy)]
pub struct Physical {
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
    pub fn new(radius: f32, mass: f32) -> Physical {
        Physical {
            velocity: math::Vector2::new(0.0, 0.0),
            max_velocity: 110.0,
            rotation: 0.0,
            radius: radius,
            mass: mass,
        }
    }
}

/// Combat represents damage, defense and attack in the game
#[derive(Debug)]
pub struct Combat {
    pub structure: i32,
    pub armour: i32,
    // lasers
    pub laser_damage: i32,
    pub laser_timer: f32,
    pub laser_velocity: f32,
    pub reload_timer: f32,
    pub time_to_reload: f32,
    pub burst_rate: i32,

    pub burst_delay: f32,
    pub burst_timer: f32,

    pub missile_damage: i32,
    pub missile_timer: f32,
    pub missile_explosion_radius: f32,
    pub missile_velocity: f32,
}

impl Component for Combat {
    type Storage = DenseVecStorage<Self>;
}

impl Combat {
    pub fn new(
        structure: i32,
        armour: i32, 
        laser_damage: i32,
        laser_timer: f32,
        laser_velocity: f32,


        missile_damage: i32,
        missile_timer: f32,
        missile_explosion_radius: f32,
        missile_velocity: f32,
    ) -> Combat {
            Combat {
                structure,
                armour,

                laser_damage,
                laser_timer,
                laser_velocity,
                reload_timer: 0.0,
                time_to_reload: 0.2,
                burst_rate: 8,

                burst_delay: 0.05,
                burst_timer: 0.0,

                missile_damage,
                missile_timer,
                missile_explosion_radius,
                missile_velocity,
            }
        }
}

/// Scoreboard contains score data
#[derive(Default)]
pub struct ScoreBoard {
    pub score_light: i32,
    pub score_dark: i32,
}

/// ScoreText contains the UI text components that display the score
pub struct ScoreText {
    pub light_text: Entity,
    pub dark_text: Entity,
}

pub struct StructureText {
    pub light_struct_text: Entity,
    pub dark_struct_text: Entity,
}

pub struct SimpleAnimation {
    start_sprite_index: usize,
    frames: usize,
    current_frame: usize,
    time_per_frame: f32,
    elapsed_time: f32,
    active: bool,
}

impl SimpleAnimation {
    pub fn new(start_sprite_index: usize, frames: usize, time_per_frame: f32,) -> SimpleAnimation {
        SimpleAnimation {
            start_sprite_index,
            frames,
            current_frame: 0,
            time_per_frame,
            elapsed_time: 0.0,
            active: true,
        }
    }
}

impl Component for SimpleAnimation {
    type Storage = DenseVecStorage<Self>;
}

/// helper function to load sprites
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

/// Initialize force field
fn initialize_force_field(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut light_transform = Transform::default();
    let mut dark_transform = Transform::default();

    // rescale fields
    light_transform.set_scale(math::Vector3::new(1024.0 / ARENA_WIDTH * 0.05, 1.0, 0.0));
    dark_transform.set_scale(math::Vector3::new(1024.0 / ARENA_WIDTH * 0.05, 1.0, 0.0));

    // Correctly position the fields.
    light_transform.set_translation_xyz(0.0, ARENA_HEIGHT / 2.0, 0.0);
    dark_transform.set_translation_xyz(ARENA_WIDTH, ARENA_HEIGHT / 2.0, 0.0);

    // Assign the sprites for the ships
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0, // ship is the first sprite in the sprite_sheet
    };

    // Create a light ship entity.
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(light_transform)
        .build();

    // Create dark ship entity.
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(dark_transform)
        .build();
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
        .with(Physical::new(48.0, 100.0))
        .with(Combat::new(150, 5, 10, 6.0, 10.0, 25, 30.0, 6.0, 5.0))
        .build();

    // Create dark ship entity.
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Ship::new(Side::Dark))
        .with(dark_transform)
        .with(Physical::new(48.0, 100.0))
        .with(Combat::new(150, 5, 10, 6.0, 10.0, 25, 30.0, 6.0, 5.0))
        .build();
}

fn initialize_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let light_transform = UiTransform::new(
        "Light".to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
        -50.0, -50.0, 1.0, 200.0, 50.0,
    );

    let dark_transform = UiTransform::new(
        "Dark".to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
        50.0, -50.0, 1.0, 200.0, 50.0,
    );


    let light_text = world
        .create_entity()
        .with(light_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            50.0,
        )).build();

        let dark_text = world
        .create_entity()
        .with(dark_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            50.0,
        )).build();

    world.insert(ScoreText { light_text, dark_text });

}

fn initialize_ship_structures(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let light_struct_ui = UiTransform::new(
        "Light".to_string(), Anchor::BottomLeft, Anchor::BottomLeft,
        50.0, 50.0, 1.0, 200.0, 50.0,
    );

    let dark_struct_ui = UiTransform::new(
        "Dark".to_string(), Anchor::BottomRight, Anchor::BottomRight,
        -50.0, 50.0, 1.0, 200.0, 50.0,
    );


    let light_struct_text = world
        .create_entity()
        .with(light_struct_ui)
        .with(UiText::new(
            font.clone(),
            "HP: 150".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            50.0,
        )).build();

        let dark_struct_text = world
        .create_entity()
        .with(dark_struct_ui)
        .with(UiText::new(
            font.clone(),
            "HP: 150".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            50.0,
        )).build();

    world.insert(StructureText { light_struct_text, dark_struct_text });

}

pub struct RandomGen;

impl RandomGen {
    // generate a random usize
    pub fn next_usize(&self) -> usize {
        use rand::Rng;
        rand::thread_rng().gen::<usize>()
    }

    pub fn next_f32(&self) -> f32 {
        use rand::Rng;
        rand::thread_rng().gen::<f32>()
    }
}

