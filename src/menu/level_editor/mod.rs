use bevy::prelude::*;

use self::{
    edit_or_new::EditLevelEditorPlugin, level_select::LevelEditorSelectPlugin,
    main_editor::LevelEditorMainPlugin, save::LevelEditorSavePlugin,
    start_input::LevelEditorStartInputPlugin,
};

mod edit_or_new;
mod level_select;
mod main_editor;
pub mod resources;
mod save;
mod start_input;
mod utils;

#[derive(Component, Clone)]
pub struct LevelEditorItem;
pub struct LevelEditorPlugin;

impl Plugin for LevelEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LevelEditorStartInputPlugin);
        app.add_plugins(LevelEditorMainPlugin);
        app.add_plugins(LevelEditorSavePlugin);
        app.add_plugins(EditLevelEditorPlugin);
        app.add_plugins(LevelEditorSelectPlugin);
    }
}
