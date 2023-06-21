use bevy::prelude::*;

mod handle_click;
mod setup;
mod exit;

use crate::{game::maps::load_starting_map, state::DisplayState, utils::delete_all_components};

use self::{
    handle_click::handle_click,
    setup::{setup, LevelSelectItem}, exit::handle_exit,
};

pub struct LevelEditorSelectPlugin;

impl Plugin for LevelEditorSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(DisplayState::LevelEditorLevelSelect)));
        app.add_system(handle_click.in_set(OnUpdate(DisplayState::LevelEditorLevelSelect)));
        app.add_systems(
            (load_starting_map, handle_exit, delete_all_components::<LevelSelectItem>)
                .chain()
                .in_schedule(OnExit(DisplayState::LevelEditorLevelSelect)),
        );
    }
}
