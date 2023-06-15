use bevy::prelude::*;

use crate::state::DisplayState;

pub fn handle_exit_to_save(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<NextState<DisplayState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_state.set(DisplayState::LevelEditorSave);
        keyboard_input.reset(KeyCode::Escape);
    }
}
