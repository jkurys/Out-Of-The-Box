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
        app.add_systems(OnEnter(DisplayState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                (handle_esc, handle_menu_click).run_if(in_state(DisplayState::MainMenu)),
            )
            .add_systems(
                OnExit(DisplayState::MainMenu),
                delete_all_components::<MainMenuItem>,
            );
    }
}
