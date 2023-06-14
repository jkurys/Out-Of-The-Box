use bevy::{app::AppExit, prelude::*};

use crate::{state::DisplayState, resources::StateStack};

pub fn handle_esc(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    app_state: ResMut<State<DisplayState>>,
    mut next_state: ResMut<NextState<DisplayState>>,
    mut state_stack: ResMut<StateStack>,
    mut app_exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if app_state.0 == DisplayState::MainMenu {
            app_exit.send(AppExit);
            return; //just in case to avoid weird behaviour before event is parsed
        }
        next_state.set(state_stack.0.pop().expect("Could not exit properly"));
        keyboard_input.reset(KeyCode::Escape);
    }
}
