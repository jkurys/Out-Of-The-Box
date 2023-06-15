use bevy::prelude::*;

pub mod level_editor;
mod level_select;
mod main_menu;
mod resources;
mod sprite_select;
use crate::resources::StateStack;
use crate::{
    exit::handle_esc,
    state::DisplayState,
    utils::delete_all_components,
};

use main_menu::{handle_menu_click, setup_main_menu};

use level_editor::editor::*;

use level_editor::tabs::*;

use self::level_editor::resources::BoardSize;
use self::level_select::LevelSelectPlugin;
use self::sprite_select::SpriteSelectPlugin;
use self::{
    main_menu::MainMenuItem,
    resources::LevelNames,
    level_editor::{save::{handle_exit_to_save, setup_file_name_getter, handle_file_get, save_board_to_file, LevelEditorSaveItem}, events::FileSavedEvent, editor_size_select::{setup_level_editor, handle_level_editor_input}},
};

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FileSavedEvent>();
        app.add_plugin(SpriteSelectPlugin);
        app.add_plugin(LevelSelectPlugin);

        app.add_system(setup_main_menu.in_schedule(OnEnter(DisplayState::MainMenu)))
            .add_systems((handle_esc, handle_menu_click).in_set(OnUpdate(DisplayState::MainMenu)))
            .add_system(delete_all_components::<MainMenuItem>.in_schedule(OnExit(DisplayState::MainMenu)));
        
        app.add_system(setup_level_editor.in_schedule(OnEnter(DisplayState::LevelEditorInput)))
            .add_systems((handle_level_editor_input, handle_esc).in_set(OnUpdate(DisplayState::LevelEditorInput)))
            .add_system(delete_all_components::<LevelEditorItem>.in_schedule(OnExit(DisplayState::LevelEditorInput)));

        app.add_system(setup_file_name_getter.in_schedule(OnEnter(DisplayState::LevelEditorSave)))
            .add_systems((
                handle_file_get,
                save_board_to_file,
                handle_esc,
            ).in_set(OnUpdate(DisplayState::LevelEditorSave)))
            .add_system(delete_all_components::<LevelEditorSaveItem>.in_schedule(OnExit(DisplayState::LevelEditorSave)));

        app.add_system(setup_level_editor_board.in_schedule(OnEnter(DisplayState::LevelEditorBoard)))
            .add_systems((
                handle_level_editor_click,
                handle_plus_click,
                handle_tab_click,
                handle_exit_to_save,
            ).in_set(OnUpdate(DisplayState::LevelEditorBoard)))
            .add_system(delete_all_components::<LevelEditorItem>.in_schedule(OnExit(DisplayState::LevelEditorBoard)));

        app.init_resource::<StateStack>();
        app.insert_resource(LevelNames(Vec::new()));
        app.init_resource::<BoardSize>();
    }
}

pub fn spawn_button<T>(parent: &mut ChildBuilder, entity: T, menu_font: Handle<Font>, value: &str)
where
    T: Component,
{
    parent
        .spawn(ButtonBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(10.0),
                    height: Val::Px(30.0),
                },
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        })
        .insert(entity)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    value,
                    TextStyle {
                        font_size: 15.0,
                        color: Color::BLACK,
                        font: menu_font,
                    },
                )
                .with_text_alignment(TextAlignment::Center),
            );
        });
}
