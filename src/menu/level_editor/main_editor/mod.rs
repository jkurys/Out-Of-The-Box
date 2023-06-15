use bevy::prelude::*;

use crate::{state::DisplayState, utils::delete_all_components, game::game_objects::Position};

use self::{setup::setup_level_editor_board, handle_click::handle_level_editor_click, plus::handle_plus_click, tabs::handle_tab_click, exit::handle_exit_to_save};

use super::LevelEditorItem;

#[derive(Component)]
pub struct LevelEditorTabs;

#[derive(Component)]
pub struct LevelEditorTab(pub usize);

#[derive(Component)]
pub struct LevelEditorTabPlus;

#[derive(Component)]
pub struct LevelEditorChangable(pub Position);

mod setup;
mod tabs;
mod handle_click;
mod plus;
mod exit;
mod styles;

pub struct LevelEditorMainPlugin;

impl Plugin for LevelEditorMainPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_level_editor_board.in_schedule(OnEnter(DisplayState::LevelEditorBoard)))
            .add_systems((
                handle_level_editor_click,
                handle_plus_click,
                handle_tab_click,
                handle_exit_to_save,
            ).in_set(OnUpdate(DisplayState::LevelEditorBoard)))
            .add_system(delete_all_components::<LevelEditorItem>.in_schedule(OnExit(DisplayState::LevelEditorBoard)));
    }
}

