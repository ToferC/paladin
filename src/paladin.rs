use amethyst::{
    assets::{Loader, ProgressCounter},
    core::{transform::Transform},
    ecs::prelude::{Entity},
    prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
    renderer::Camera,
};

use crate::audio::{initialize_audio};
use crate::resources::assets::*;

use crate::components::{LaserRes, Laser, initialise_ships};

pub const ARENA_HEIGHT: f32 = 1024.0;
pub const ARENA_WIDTH: f32 = 1600.0;

pub const LASER_RADIUS: f32 = 4.0;

#[derive(Default)]
pub struct Paladin;

impl SimpleState for Paladin {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        let mut progress_counter = ProgressCounter::new();

        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        // `spritesheet` is the layout of the sprites on the image;
        // `texture` is the pixel data.

        world.register::<Laser>();

        initialize_audio(world);

        let _progress_counter = Some(load_assets(
            world,
            vec![
                AssetType::LaserImpact,
            ],
        ));

        LaserRes::initialise(world);
        world.insert(RandomGen);

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

fn initialize_ship_hp_ui(world: &mut World) {
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

