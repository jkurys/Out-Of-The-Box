use bevy::prelude::*;

use self::{start_input::LevelEditorStartInputPlugin, main_editor::LevelEditorMainPlugin, save::LevelEditorSavePlugin};

pub mod resources;
mod save;
mod utils;
mod start_input;
mod main_editor;

#[derive(Component)]
pub struct LevelEditorItem;
pub struct LevelEditorPlugin;

impl Plugin for LevelEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LevelEditorStartInputPlugin);
        app.add_plugin(LevelEditorMainPlugin);
        app.add_plugin(LevelEditorSavePlugin);
    }
}
