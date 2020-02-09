use amethyst::{
    assets::{Loader},
    ecs::prelude::{Entity},
    prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

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

pub fn initialize_scoreboard(world: &mut World) {
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

pub fn initialize_ship_hp_ui(world: &mut World) {
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