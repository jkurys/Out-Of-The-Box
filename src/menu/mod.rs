use bevy::prelude::*;

mod level_editor;
mod level_select;
mod main_menu;
mod resources;
mod sprite_select;
use crate::{
    consts::{MAX_HEIGHT, MAX_WIDTH},
    exit::handle_esc,
    state::DisplayState,
    utils::delete_all_components,
};

use level_select::{handle_level_click, setup_level_select};
use main_menu::{handle_menu_click, setup_main_menu};

use level_editor::editor::*;

use level_editor::tabs::*;

use level_editor::resources::LevelEditorBoard;

use self::{
    level_select::LevelSelectItem,
    main_menu::MainMenuItem,
    resources::LevelNames,
    sprite_select::{handle_sprite_click, setup_sprite_select, SpriteSelectItem}, level_editor::{save::{handle_exit_to_save, setup_file_name_getter, handle_file_get, save_board_to_file, LevelEditorSaveItem}, events::FileSavedEvent, editor_size_select::{setup_level_editor, handle_level_editor_input}},
};

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FileSavedEvent>();
        app.add_system_set(
            SystemSet::on_enter(DisplayState::MainMenu).with_system(setup_main_menu),
        )
        .add_system_set(SystemSet::on_resume(DisplayState::MainMenu).with_system(setup_main_menu))
        .add_system_set(
            SystemSet::on_update(DisplayState::MainMenu)
                .with_system(handle_menu_click)
                .with_system(handle_esc),
        )
        .add_system_set(
            SystemSet::on_pause(DisplayState::MainMenu)
                .with_system(delete_all_components::<MainMenuItem>),
        )
        .add_system_set(
            SystemSet::on_exit(DisplayState::MainMenu)
                .with_system(delete_all_components::<MainMenuItem>),
        );

        app.add_system_set(
            SystemSet::on_enter(DisplayState::LevelSelect).with_system(setup_level_select),
        )
        .add_system_set(
            SystemSet::on_resume(DisplayState::LevelSelect).with_system(setup_level_select),
        )
        .add_system_set(
            SystemSet::on_update(DisplayState::LevelSelect)
                .with_system(handle_level_click)
                .with_system(handle_esc),
        )
        .add_system_set(
            SystemSet::on_exit(DisplayState::LevelSelect)
                .with_system(delete_all_components::<LevelSelectItem>),
        )
        .add_system_set(
            SystemSet::on_pause(DisplayState::LevelSelect)
                .with_system(delete_all_components::<LevelSelectItem>),
        );

        app.add_system_set(
            SystemSet::on_enter(DisplayState::SpriteSelect).with_system(setup_sprite_select),
        )
        .add_system_set(
            SystemSet::on_resume(DisplayState::SpriteSelect).with_system(setup_sprite_select),
        )
        .add_system_set(
            SystemSet::on_update(DisplayState::SpriteSelect)
                .with_system(handle_sprite_click)
                .with_system(handle_esc),
        )
        .add_system_set(
            SystemSet::on_exit(DisplayState::SpriteSelect)
                .with_system(delete_all_components::<SpriteSelectItem>),
        )
        .add_system_set(
            SystemSet::on_pause(DisplayState::SpriteSelect)
                .with_system(delete_all_components::<SpriteSelectItem>),
        );
        app.add_system_set(
            SystemSet::on_enter(DisplayState::LevelEditorInput).with_system(setup_level_editor),
        )
        .add_system_set(
            SystemSet::on_resume(DisplayState::LevelEditorInput).with_system(setup_level_editor),
        )
        .add_system_set(
            SystemSet::on_update(DisplayState::LevelEditorInput)
                .with_system(handle_level_editor_input)
                .with_system(handle_esc),
        )
        .add_system_set(
            SystemSet::on_exit(DisplayState::LevelEditorInput)
                .with_system(delete_all_components::<LevelEditorItem>),
        )
        .add_system_set(
            SystemSet::on_pause(DisplayState::LevelEditorInput)
                .with_system(delete_all_components::<LevelEditorItem>),
        );
        app.add_system_set(
            SystemSet::on_enter(DisplayState::LevelEditorSave)
                .with_system(setup_file_name_getter)
        )
        .add_system_set(
            SystemSet::on_resume(DisplayState::LevelEditorSave)
                .with_system(setup_file_name_getter)
        )
        .add_system_set(
            SystemSet::on_update(DisplayState::LevelEditorSave)
                .with_system(handle_file_get)
                .with_system(save_board_to_file)
                .with_system(handle_esc)
        )
        .add_system_set(
            SystemSet::on_exit(DisplayState::LevelEditorSave)
                .with_system(delete_all_components::<LevelEditorSaveItem>)
        )
        .add_system_set(
            SystemSet::on_pause(DisplayState::LevelEditorSave)
                .with_system(delete_all_components::<LevelEditorSaveItem>)
        );
        for i in 1..MAX_WIDTH {
            for j in 1..MAX_HEIGHT {
                app.add_system_set(
                    SystemSet::on_enter(DisplayState::LevelEditorBoard(i, j))
                        .with_system(setup_level_editor_board),
                )
                .add_system_set(
                    SystemSet::on_resume(DisplayState::LevelEditorBoard(i, j))
                        .with_system(setup_level_editor_board),
                )
                .add_system_set(
                    SystemSet::on_update(DisplayState::LevelEditorBoard(i, j))
                        .with_system(handle_level_editor_click)
                        .with_system(handle_tab_click)
                        .with_system(handle_plus_click)
                        // .with_system(handle_esc),
                        .with_system(handle_exit_to_save)
                )
                .add_system_set(
                    SystemSet::on_exit(DisplayState::LevelEditorBoard(i, j))
                        .with_system(delete_all_components::<LevelEditorItem>)
                        // .with_system(save_board_to_file),
                )
                .add_system_set(
                    SystemSet::on_pause(DisplayState::LevelEditorBoard(i, j))
                        .with_system(delete_all_components::<LevelEditorItem>)
                        // .with_system(save_board_to_file),
                );
            }
        }
        app.init_resource::<LevelEditorBoard>();
        app.insert_resource(LevelNames(Vec::new()));
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
                .with_text_alignment(TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }),
            );
        });
}
