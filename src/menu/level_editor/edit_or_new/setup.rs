use bevy::prelude::*;

use crate::{consts::MAIN_MENU_FONT, menu::spawn_button};

#[derive(Component)]
pub struct LevelEditorChooseElement;

#[derive(Component, Clone, Copy)]
pub enum ButtonType {
    New,
    Edit,
    Back,
}

pub fn setup_level_editor_choose(mut commands: Commands, asset_server: Res<AssetServer>) {
    let menu_font = asset_server.load(MAIN_MENU_FONT);
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                size: Size {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                },
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .insert(LevelEditorChooseElement)
        .with_children(|parent| {
            spawn_button(parent, ButtonType::New, menu_font.clone(), "New");
            spawn_button(parent, ButtonType::Edit, menu_font.clone(), "Edit");
            spawn_button(parent, ButtonType::Back, menu_font.clone(), "Back");
        });
}
