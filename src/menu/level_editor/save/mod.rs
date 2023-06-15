use bevy::prelude::*;

use crate::{exit::handle_esc, utils::delete_all_components, state::DisplayState};

use self::{setup::setup_file_name_getter, handle_click::handle_file_get, exit::save_board_to_file, events::FileSavedEvent};

mod setup;
mod handle_click;
mod exit;
mod events;

#[derive(Component)]
pub struct LevelEditorSaveItem;

#[derive(Component)]
pub struct LevelEditorFileName;

pub struct LevelEditorSavePlugin;

impl Plugin for LevelEditorSavePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FileSavedEvent>();
        app.add_system(setup_file_name_getter.in_schedule(OnEnter(DisplayState::LevelEditorSave)))
            .add_systems((
                handle_file_get,
                save_board_to_file,
                handle_esc,
            ).in_set(OnUpdate(DisplayState::LevelEditorSave)))
            .add_system(delete_all_components::<LevelEditorSaveItem>.in_schedule(OnExit(DisplayState::LevelEditorSave)));

    }
}