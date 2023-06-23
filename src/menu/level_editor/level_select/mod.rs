use bevy::prelude::*;

mod exit;
mod handle_click;
mod setup;

use crate::{game::maps::load_starting_map, state::DisplayState, utils::delete_all_components, resources::CurrentLevel};

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
        app.add_system(setup.in_schedule(OnEnter(DisplayState::LevelEditorLevelSelect)));
        app.add_system(handle_click.in_set(OnUpdate(DisplayState::LevelEditorLevelSelect)));
        app.add_systems(
            (
                handle_exit,
                delete_all_components::<LevelSelectItem>,
                load_starting_map.run_if(exited_to_level),
            )
                .chain()
                .in_schedule(OnExit(DisplayState::LevelEditorLevelSelect)),
        );
    }
}
