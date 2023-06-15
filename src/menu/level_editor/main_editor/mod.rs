use bevy::prelude::*;

use crate::{game::game_objects::Position, state::DisplayState, utils::delete_all_components};

use self::{
    exit::handle_exit_to_save, handle_click::handle_level_editor_click, plus::handle_plus_click,
    setup::setup_level_editor_board, tabs::handle_tab_click,
};

use super::LevelEditorItem;

#[derive(Component)]
pub struct LevelEditorTabs;

#[derive(Component)]
pub struct LevelEditorTab(pub usize);

#[derive(Component)]
pub struct LevelEditorTabPlus;

#[derive(Component)]
pub struct LevelEditorChangable(pub Position);

mod exit;
mod handle_click;
mod plus;
mod setup;
mod styles;
mod tabs;

pub struct LevelEditorMainPlugin;

impl Plugin for LevelEditorMainPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            setup_level_editor_board.in_schedule(OnEnter(DisplayState::LevelEditorBoard)),
        )
        .add_systems(
            (
                handle_level_editor_click,
                handle_plus_click,
                handle_tab_click,
                handle_exit_to_save,
            )
                .in_set(OnUpdate(DisplayState::LevelEditorBoard)),
        )
        .add_system(
            delete_all_components::<LevelEditorItem>
                .in_schedule(OnExit(DisplayState::LevelEditorBoard)),
        );
    }
}
