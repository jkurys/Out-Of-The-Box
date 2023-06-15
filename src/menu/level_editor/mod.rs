use bevy::prelude::*;

use self::{
    main_editor::LevelEditorMainPlugin, save::LevelEditorSavePlugin,
    start_input::LevelEditorStartInputPlugin,
};

mod main_editor;
pub mod resources;
mod save;
mod start_input;
mod utils;

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
