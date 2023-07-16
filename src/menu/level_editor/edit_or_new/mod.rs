use bevy::prelude::*;

use crate::state::DisplayState;
use crate::utils::delete_all_components;

use self::handle_click::handle_click;
use self::setup::{setup_level_editor_choose, LevelEditorChooseElement};

mod handle_click;
mod setup;
pub struct EditLevelEditorPlugin;

impl Plugin for EditLevelEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(DisplayState::LevelEditorSelect),
            setup_level_editor_choose,
        );

        app.add_systems(
            Update,
            handle_click.run_if(in_state(DisplayState::LevelEditorSelect))
        );

        app.add_systems(
            OnExit(DisplayState::LevelEditorSelect),
            delete_all_components::<LevelEditorChooseElement>
        );
    }
}
