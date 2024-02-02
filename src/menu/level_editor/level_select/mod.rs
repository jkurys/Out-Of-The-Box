use bevy::prelude::*;

mod exit;
mod handle_click;
mod setup;

use crate::{
    game::maps::load_starting_map, resources::CurrentLevel, state::DisplayState,
    utils::delete_all_components,
};

use self::{
    exit::handle_exit,
    handle_click::handle_click,
    setup::{setup, LevelSelectItem},
};

pub fn exited_to_level(current_level: Res<CurrentLevel>) -> bool {
    current_level.is_in_level
}

pub struct LevelEditorSelectPlugin;

impl Plugin for LevelEditorSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(DisplayState::LevelEditorLevelSelect), setup);
        app.add_systems(
            Update,
            handle_click.run_if(in_state(DisplayState::LevelEditorLevelSelect)),
        );
        app.add_systems(
            OnExit(DisplayState::LevelEditorLevelSelect),
            (
                delete_all_components::<LevelSelectItem>,
                load_starting_map.run_if(exited_to_level),
                handle_exit,
            )
                .chain(),
        );
    }
}
