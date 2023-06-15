use bevy::prelude::*;

pub mod level_editor;
mod level_select;
mod main_menu;
mod resources;
mod sprite_select;
use crate::resources::StateStack;

use self::level_editor::resources::BoardSize;
use self::level_editor::LevelEditorPlugin;
use self::level_select::LevelSelectPlugin;
use self::main_menu::MainMenuPlugin;
use self::resources::LevelNames;
use self::sprite_select::SpriteSelectPlugin;

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SpriteSelectPlugin);
        app.add_plugin(LevelSelectPlugin);
        app.add_plugin(MainMenuPlugin);
        app.add_plugin(LevelEditorPlugin);

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
