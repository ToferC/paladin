use amethyst::{
    ecs::prelude::Entity,
    core::transform::Parent,
    input::{is_close_requested, is_key_down, is_mouse_button_down},
    prelude::*,
    ui::UiCreator,
    winit::{MouseButton, VirtualKeyCode},
};

use super::utils::delete_hierarchy;
use crate::resources::{AssetType, load_assets};
use super::game::RandomGen;
use crate::audio::initialize_audio;

#[derive(Default, Debug)]
pub struct WelcomeScreen {
    ui_handle: Option<Entity>,
}

impl SimpleState for WelcomeScreen {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialize_audio(world);

        world.insert(RandomGen);

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

        world.register::<Parent>();

        self.ui_handle =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/welcome.ron", ())));
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_mouse_button_down(&event, MouseButton::Left) {
                    log::info!("[Trans::Switch] Switching to MainMenu!");
                    Trans::Switch(Box::new(super::menu::MainMenu::default()))
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(handler) = self.ui_handle {
            delete_hierarchy(handler, data.world).expect("Failed to remove WelcomeScreen");
        }
        self.ui_handle = None;
    }
}