use bevy::prelude::*;

use crate::{exit::handle_esc, utils::delete_all_components, state::DisplayState};

use self::{setup::setup_main_menu, handle_click::handle_menu_click};

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
            .add_system(delete_all_components::<MainMenuItem>.in_schedule(OnExit(DisplayState::MainMenu)));
    }
}
