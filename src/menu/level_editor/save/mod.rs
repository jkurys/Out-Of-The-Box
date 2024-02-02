use bevy::prelude::*;

use crate::{exit::handle_esc, state::DisplayState, utils::delete_all_components};

use self::{
    events::FileSavedEvent,
    exit::{clear_board, save_board_to_file},
    handle_click::handle_file_get,
    setup::setup_file_name_getter,
};

mod events;
mod exit;
mod handle_click;
mod setup;

#[derive(Component)]
pub struct LevelEditorSaveItem;

#[derive(Component)]
pub struct LevelEditorFileName;

pub struct LevelEditorSavePlugin;

impl Plugin for LevelEditorSavePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FileSavedEvent>();
        app.add_systems(
            OnEnter(DisplayState::LevelEditorSave),
            setup_file_name_getter,
        )
        .add_systems(
            Update,
            (handle_file_get, save_board_to_file, handle_esc)
                .run_if(in_state(DisplayState::LevelEditorSave)),
        )
        .add_systems(
            OnExit(DisplayState::LevelEditorSave),
            (delete_all_components::<LevelEditorSaveItem>, clear_board),
        );
    }
}
