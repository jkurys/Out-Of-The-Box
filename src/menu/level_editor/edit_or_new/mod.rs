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
        app.add_system(
            setup_level_editor_choose.in_schedule(OnEnter(DisplayState::LevelEditorSelect)),
        );

        app.add_system(handle_click.in_set(OnUpdate(DisplayState::LevelEditorSelect)));

        app.add_system(
            delete_all_components::<LevelEditorChooseElement>
                .in_schedule(OnExit(DisplayState::LevelEditorSelect)),
        );
    }
}
