//! Pong Tutorial 2

mod paladin;
mod systems;
mod audio;
mod resources;
mod components;
mod states;

extern crate specs_derive;

use amethyst::{
    animation::AnimationBundle,
    assets::{PrefabLoaderSystem, PrefabLoaderSystemDesc},
    core::TransformBundle,
    prelude::*,
    audio::{AudioBundle, DjSystemDesc},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
        sprite::SpriteRender,
    },
    ui::{RenderUi, UiBundle},
    input::{InputBundle, StringBindings},
    utils::application_root_dir,
};

use audio::Music;
use crate::states::Game;
use crate::components::{AnimationPrefabData, AnimationId};
use systems::*;

const BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    // This line is not mentioned in the pong tutorial as it is specific to the context
    // of the git repository. It only is a different location to load the assets from.
    let assets_dir = app_root.join("assets");

    let game_data = GameDataBuilder::default()
        // Prefabbundlecar
        .with_system_desc(
            PrefabLoaderSystemDesc::<AnimationPrefabData>::default(),
            "scene_loader",
            &[],
            )
        // Animation
        .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ))?
        // Add the transform bundle which handles tracking entity positions
        .with_bundle(TransformBundle::new()
            .with_dep(&["sprite_animation_control", "sprite_sampler_interpolation"]),
        )?
        // Add input bundle
        .with_bundle(input_bundle)?
        // Add bundle for UI handling
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_system_desc(
            DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        )
        // Add systems
        .with(systems::MovementSystem, "movement_system", &["input_system"]
        )
        .with(
            systems::LaserSystem, "laser_system", &["input_system"]
        )
        .with(
            systems::PhysicsSystem, "physics_system", &["movement_system"]
        )
        .with(
            systems::CollisionSystem,
            "collision_system",
            &["laser_system", "physics_system"],
        )
        .with(
            systems::WinnerSystem,
            "winner_system",
            &["movement_system", "physics_system"],
        )
        .with(
            LaserImpactAnimationSystem,
            "laser_impact_animation_system",
            &["laser_system", "collision_system"],
        )
        .with(AnimationControlSystem,
            "animation_control_system",
            &["laser_impact_animation_system"]
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and
                // drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear(BACKGROUND_COLOR),
                )
                // RenderFlat2D plugin is used to render entities with `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::new(
        assets_dir, 
        Game::default(), 
        game_data)?;

    game.run();

    Ok(())
}
