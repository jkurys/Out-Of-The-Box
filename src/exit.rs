use bevy::{app::AppExit, prelude::*};

use crate::{resources::StateStack, state::DisplayState};

pub fn handle_esc(
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
    state: Res<State<DisplayState>>,
    mut next_state: ResMut<NextState<DisplayState>>,
    mut state_stack: ResMut<StateStack>,
    mut app_exit: EventWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if state.get() == &DisplayState::MainMenu {
            app_exit.send(AppExit);
            return; //just in case to avoid weird behaviour before event is parsed
        }
        next_state.set(state_stack.0.pop().unwrap_or(DisplayState::MainMenu));
        keyboard.reset(KeyCode::Escape);
    }
}
