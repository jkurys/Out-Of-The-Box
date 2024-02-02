use bevy::prelude::*;

use crate::{exit::handle_esc, state::DisplayState, utils::delete_all_components};

use self::{handle_click::handle_level_click, setup::setup_level_select};

mod handle_click;
mod setup;

#[derive(Component)]
pub struct LevelSelectItem;

#[derive(Component)]
pub enum LevelSelectItemType {
    Level(usize),
    Back,
}

pub struct LevelSelectPlugin;

impl Plugin for LevelSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(DisplayState::LevelSelect), setup_level_select)
            .add_systems(
                Update,
                (handle_level_click, handle_esc).run_if(in_state(DisplayState::LevelSelect)),
            )
            .add_systems(
                OnExit(DisplayState::LevelSelect),
                delete_all_components::<LevelSelectItem>,
            );
    }
}
