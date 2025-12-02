use bevy::prelude::*;

use crate::{
    game::display::{
        background::render_board,
        border::insert_border,
        despawn_board,
    },
    state::DisplayState,
    utils::delete_all_components,
};

use self::{
    exit::handle_exit_to_save,
    handle_click::handle_level_editor_click,
    setup::{set_board_size, setup_level_editor_board},
    highlight::handle_highlight,
};

use super::LevelEditorItem;
mod exit;
mod handle_click;
mod plus;
mod highlight;
mod setup;
mod tabs;

pub struct LevelEditorMainPlugin;

impl Plugin for LevelEditorMainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(DisplayState::LevelEditorBoard),
            (
                set_board_size,
                setup_level_editor_board,
                render_board,
                insert_border,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                handle_level_editor_click,
                despawn_board,
                render_board,
                handle_highlight,
                handle_exit_to_save,
            )
                .chain()
                .run_if(in_state(DisplayState::LevelEditorBoard)),
        )
        .add_systems(
            OnExit(DisplayState::LevelEditorBoard),
            delete_all_components::<LevelEditorItem>,
        );
    }
}
