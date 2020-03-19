use amethyst::{
    animation::AnimationBundle,
    core::{TransformBundle, SystemExt},
    prelude::*,    
    core::{transform::Transform, Parent, Time},
    ecs::prelude::{Entity, WorldExt, Dispatcher, DispatcherBuilder},
    prelude::*,
    input::{VirtualKeyCode, is_key_down, is_close_requested},
    ui::{UiCreator, UiFinder, UiText},
    utils::fps_counter::FpsCounter,
    renderer::Camera,
};

use crate::components::{AnimationPrefabData, AnimationId};
use crate::systems::{CollisionSystem, MovementSystem, LaserSystem, PhysicsSystem,
                LaserImpactAnimationSystem, WinnerSystem, AnimationControlSystem};

use crate::audio::{initialize_audio, Music};
use crate::resources::assets::*;

use crate::components::{initialise_ships};
use crate::components::{initialize_scoreboard, initialize_ship_hp_ui};

use super::pause::PauseMenuState;
use super::utils::delete_hierarchy;

pub const ARENA_HEIGHT: f32 = 1024.0 * 1.0;
pub const ARENA_WIDTH: f32 = 1600.0 * 1.0;

pub const LASER_RADIUS: f32 = 4.0;

#[derive(PartialEq)]
pub enum CurrentState {
    Disabled,
    Enabled,
}

impl Default for CurrentState {
    fn default() -> Self {
        CurrentState::Disabled
    }
}

pub struct Game {
    pub player_count: u8,
    paused: bool,
    ui_root: Option<Entity>,
    fps_display: Option<Entity>,
    text: Option<Entity>,
    dispatcher: Dispatcher<'static, 'static>,
}

impl Game {
    pub fn new(world: &mut World) -> Self {

        Game {
            player_count: 2u8,
            paused: false,
            ui_root: None,
            fps_display: None,
            text: None,
            dispatcher: DispatcherBuilder::new()

            // Add systems
            .with(MovementSystem.pausable(CurrentState::Disabled), 
                "movement_system", &[]
            )
            .with(
                LaserSystem.pausable(CurrentState::Disabled),
                "laser_system", &[]
            )
            .with(
                PhysicsSystem.pausable(CurrentState::Disabled),
                "physics_system", &["movement_system"]
            )
            .with(
                CollisionSystem.pausable(CurrentState::Disabled),
                "collision_system",
                &["laser_system", "physics_system", "movement_system"],
            )
            .with(
                WinnerSystem.pausable(CurrentState::Disabled),
                "winner_system",
                &["movement_system", "physics_system"],
            )
            .with(
                LaserImpactAnimationSystem.pausable(CurrentState::Disabled),
                "laser_impact_animation_system",
                &["laser_system", "collision_system"],
            )
            .with(AnimationControlSystem.pausable(CurrentState::Disabled),
                "animation_control_system",
                &["laser_impact_animation_system"]
            )
            .build(),
        }
    }
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        let mut world = data.world;
        
        self.dispatcher.setup(&mut world);
        
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

        world.maintain();
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {

        self.dispatcher.dispatch(&data.world);

        data.data.update(&data.world);
        
        Trans::None

    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Push] Pausing Game!");
                    Trans::Push(Box::new(PauseMenuState::default()))
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                log::info!(
                    "[Handle Event] You just interacted with a UI element: {:?}",
                    ui_event,
                );
                Trans::None
            }
            StateEvent::Input(input) => {
                //log::info!("Input Event detected {:?}", input);
                Trans::None
            }
        }
    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        *world.write_resource() = CurrentState::Enabled;

        self.paused = true;
        update_time_scale(self.paused, world);
        log::info!("PAUSED");
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        *world.write_resource() = CurrentState::Disabled;
        
        self.paused = false;
        update_time_scale(self.paused, world);

    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(entity) = self.ui_root {
            delete_hierarchy(entity, data.world).expect("Failed to remove game");
        }
        self.ui_root = None;
        self.fps_display = None;
        self.text = None;
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

fn update_time_scale(paused: bool, world: &mut World) {
    world
        .write_resource::<Time>()
        .set_time_scale(if paused {
            0.0
        } else {
            1.0
        });
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

