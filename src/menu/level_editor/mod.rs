use bevy::prelude::*;

use self::{
    edit_or_new::EditLevelEditorPlugin, main_editor::LevelEditorMainPlugin,
    save::LevelEditorSavePlugin, start_input::LevelEditorStartInputPlugin, level_select::LevelEditorSelectPlugin,
};

mod edit_or_new;
mod level_select;
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
        app.add_plugin(EditLevelEditorPlugin);
        app.add_plugin(LevelEditorSelectPlugin);
    }
}
