use amethyst::{
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down},
    prelude::*,
    ui::{UiCreator, UiEvent, UiEventType, UiFinder},
    winit::VirtualKeyCode,
};

use super::{
    credits::CreditsScreen,
    game::Game,
    utils::delete_hierarchy,
    welcome::WelcomeScreen,
};

use log::{info, warn};

use crate::audio::initialize_audio;

const BUTTON_START: &str = "start";
const BUTTON_LOAD: &str = "load";
const BUTTON_OPTIONS: &str = "options";
const BUTTON_CREDITS: &str = "credits";

#[derive(Default, Debug)]
pub struct MainMenu {
    ui_root: Option<Entity>,
    button_start: Option<Entity>,
    button_load: Option<Entity>,
    button_options: Option<Entity>,
    button_credits: Option<Entity>,
}

impl SimpleState for MainMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // create UI from prefab and save reference
        let world = data.world;

        self.ui_root = 
            Some(world.exec(|mut creator: UiCreator<'_>|
                creator.create("ui/menu.ron", ())));

        initialize_audio(world);

    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // only search for buttons if they have not been found yet
        let StateData { world, ..} = state_data;

        if self.button_start.is_none()
            || self.button_load.is_none()
            || self.button_options.is_none()
            || self.button_credits.is_none()
        {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_start = ui_finder.find(BUTTON_START);
                self.button_load = ui_finder.find(BUTTON_LOAD);
                self.button_options = ui_finder.find(BUTTON_OPTIONS);
                self.button_credits = ui_finder.find(BUTTON_CREDITS);
            });
        }
        Trans::None
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Switch] Switching back to WelcomeScreen!");
                    Trans::Switch(Box::new(WelcomeScreen::default()))
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_credits {
                    log::info!("[Trans::Switch] Switching to CreditsScreen!");
                    return Trans::Switch(Box::new(CreditsScreen::default()));
                }
                if Some(target) == self.button_start {
                    log::info!("[Trans::Switch] Switching to Game!");
                    return Trans::Switch(Box::new(Game::new(data.world)));
                }
                if Some(target) == self.button_load || Some(target) == self.button_options {
                    log::info!("This Button's functionality is not yet implemented!");
                }
                Trans::None
            }
            _ => Trans::None,
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {

        if let Some(entity) = self.ui_root {
            delete_hierarchy(entity, data.world).expect("failed to remove MainMenu");
        }
        self.ui_root = None;
        self.button_start = None;
        self.button_load = None;
        self.button_options = None;
        self.button_credits = None;
    }
}