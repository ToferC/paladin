use amethyst::{
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down},
    prelude::*,
    ui::{UiCreator, UiEvent, UiEventType, UiFinder},
    winit::VirtualKeyCode,
};

const BUTTON_START: &str = "start";
const BUTTON_CREDITS: &str = "credits";

#[derive(Default, Debug)]
pub struct MainMenu {
    ui_root: Option<Entity>,
    button_start: Option<Entity>,
    button_credits: Option<Entity>,
}

impl SimpleState for MainMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // Create UI from Prefab and save reference
        let world = data.world;

        self.ui_root = 
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())));
    }
}