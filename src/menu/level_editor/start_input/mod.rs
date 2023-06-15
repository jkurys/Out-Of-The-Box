use bevy::prelude::*;

use crate::{utils::delete_all_components, exit::handle_esc, state::DisplayState};

use self::{setup::setup_level_editor, handle_click::handle_level_editor_input};

use super::LevelEditorItem;

mod setup;
mod handle_click;

#[derive(Component)]
pub struct LevelEditorStartingPrompt;

#[derive(Component)]
pub struct LevelEditorInputNumber;

pub struct LevelEditorStartInputPlugin;

impl Plugin for LevelEditorStartInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_level_editor.in_schedule(OnEnter(DisplayState::LevelEditorInput)))
            .add_systems((handle_level_editor_input, handle_esc).in_set(OnUpdate(DisplayState::LevelEditorInput)))
            .add_system(delete_all_components::<LevelEditorItem>.in_schedule(OnExit(DisplayState::LevelEditorInput)));
    }
}
