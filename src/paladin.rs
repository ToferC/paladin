use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{transform::Transform, math},
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

const ARENA_HEIGHT: f32 = 100.0;
const ARENA_WIDTH: f32 = 100.0;

const SHIP_HEIGHT: f32 = 16.0;
const SHIP_WIDTH: f32 = 16.0;

pub struct Paladin;

impl SimpleState for Paladin {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        // `spritesheet` is the layout of the sprites on the image;
        // `texture` is the pixel data.
        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Ship>();

        initialise_ships(world, sprite_sheet_handle);
        initialise_camera(world);
    }
}

#[derive(PartialEq, Eq)]
enum Side {
    Light,
    Dark,
}

struct Ship {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Ship {
    fn new(side: Side) -> Ship {
        Ship {
            side,
            width: SHIP_WIDTH,
            height: SHIP_HEIGHT,
        }
    }
}

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/ship_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/ship_spritesheet.ron",
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
    light_transform.set_scale(math::Vector3::new(0.02, 0.02, 0.02));
    dark_transform.set_scale(math::Vector3::new(0.02, 0.02, 0.02));

    // rotate ships
    light_transform.rotate_2d(1.60);
    dark_transform.rotate_2d(-1.60);


    // Correctly position the ships.
    let y = ARENA_HEIGHT / 2.0;
    light_transform.set_translation_xyz(SHIP_WIDTH * 0.5, y, 0.0);
    dark_transform.set_translation_xyz(ARENA_WIDTH - SHIP_WIDTH * 0.5, y, 0.0);

    // Assign the sprites for the ships
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0, // ship is the first sprite in the sprite_sheet
    };

    // Create a light plank entity.
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Ship::new(Side::Light))
        .with(light_transform)
        .build();

    // Create dark plank entity.
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Ship::new(Side::Dark))
        .with(dark_transform)
        .build();
}
