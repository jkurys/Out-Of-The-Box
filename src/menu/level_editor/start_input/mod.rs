use bevy::prelude::*;

use crate::{exit::handle_esc, state::DisplayState, utils::delete_all_components};

use self::{handle_click::handle_level_editor_input, setup::setup_level_editor};

use super::LevelEditorItem;

mod handle_click;
mod setup;

#[derive(Component)]
pub struct LevelEditorStartingPrompt;

#[derive(Component)]
pub struct LevelEditorInputNumber;

pub struct LevelEditorStartInputPlugin;

impl Plugin for LevelEditorStartInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(DisplayState::LevelEditorInput),
            setup_level_editor
        )
            .add_systems(
                Update,
                (handle_level_editor_input, handle_esc)
                   .run_if(in_state(DisplayState::LevelEditorInput)),
            )
            .add_systems(
                OnExit(DisplayState::LevelEditorInput),
                delete_all_components::<LevelEditorItem>
            );
    }
}
