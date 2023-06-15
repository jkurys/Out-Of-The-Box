use bevy::prelude::*;

use crate::{board::Board, state::DisplayState};

use super::LevelEditorTab;

pub fn handle_tab_click(
    mut tab_query: Query<
        (&LevelEditorTab, &Interaction, &mut BackgroundColor),
        With<LevelEditorTab>,
    >,
    mut boards: ResMut<Board>,
    mut app_state: ResMut<NextState<DisplayState>>
) {
    for (tab_num, interaction, mut color) in tab_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                boards.set_current_map(tab_num.0 - 1);
                app_state.set(DisplayState::LevelEditorInput);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::rgb(0.25, 0.25, 1.));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::MIDNIGHT_BLUE);
            }
        }
    }
}