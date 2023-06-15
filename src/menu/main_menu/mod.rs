use bevy::prelude::*;

use crate::{exit::handle_esc, state::DisplayState, utils::delete_all_components};

use self::{handle_click::handle_menu_click, setup::setup_main_menu};

mod handle_click;
mod setup;

#[derive(Component)]
pub struct MainMenuItem;

#[derive(Component)]
pub enum MenuItemType {
    LevelSelect,
    Exit,
    SpriteSelect,
    LevelEditor,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_main_menu.in_schedule(OnEnter(DisplayState::MainMenu)))
            .add_systems((handle_esc, handle_menu_click).in_set(OnUpdate(DisplayState::MainMenu)))
            .add_system(
                delete_all_components::<MainMenuItem>.in_schedule(OnExit(DisplayState::MainMenu)),
            );
    }
}
