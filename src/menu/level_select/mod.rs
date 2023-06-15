use bevy::prelude::*;

use crate::{state::DisplayState, utils::delete_all_components, exit::handle_esc};

use self::{setup::setup_level_select, handle_click::handle_level_click};

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
        app.add_system(setup_level_select.in_schedule(OnEnter(DisplayState::LevelSelect)))
            .add_systems((handle_level_click, handle_esc).in_set(OnUpdate(DisplayState::LevelSelect)))
            .add_system(delete_all_components::<LevelSelectItem>.in_schedule(OnExit(DisplayState::LevelSelect)));
    }
}