use amethyst::{
    assets::{ProgressCounter},
    core::{transform::Transform, Parent},
    prelude::*,
    renderer::Camera,
};

use crate::audio::{initialize_audio};
use crate::resources::assets::*;

use crate::components::{initialise_ships};
use crate::components::{initialize_scoreboard, initialize_ship_hp_ui};

pub const ARENA_HEIGHT: f32 = 1024.0 * 1.2;
pub const ARENA_WIDTH: f32 = 1600.0 * 1.2;

pub const LASER_RADIUS: f32 = 4.0;

#[derive(Default)]
pub struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        // `spritesheet` is the layout of the sprites on the image;
        // `texture` is the pixel data.

        initialize_audio(world);

        let _progress_counter = Some(load_assets(
            world,
            vec![
                AssetType::LaserImpact,
                AssetType::Thrust,
                AssetType::LaserLight,
                AssetType::LaserDark,
                AssetType::LightShip,
                AssetType::DarkShip,
            ],
        ));

        world.insert(RandomGen);

        world.register::<Parent>();

        initialize_scoreboard(world);
        initialize_ship_hp_ui(world);

        initialise_ships(world);
        initialise_camera(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {

        let world = &mut data.world;

        world.maintain();

        Trans::None
    }
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

