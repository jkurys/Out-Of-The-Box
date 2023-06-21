use std::fs::read_dir;

use bevy::prelude::*;

use crate::{
    consts::MAIN_MENU_FONT,
    menu::{resources::LevelNames, spawn_button},
};

#[derive(Component)]
pub struct LevelSelectItem;

#[derive(Component, Clone, Copy)]
pub enum ButtonType {
    Level(usize),
    Back,
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut level_names: ResMut<LevelNames>,
) {
    let paths = read_dir("./assets/maps").unwrap();
    let mut file_amount = 0;
    let mut file_paths = Vec::new();
    let mut first_name = "".to_string();
    for path in paths {
        let path_str = path
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        if first_name == "".to_string() {
            first_name = path_str.clone();
        }
        file_paths.push(path_str);
        file_amount += 1;
    }
    let menu_font = asset_server.load(MAIN_MENU_FONT);
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::BLACK),
            visibility: Visibility::Visible,
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        })
        .insert(LevelSelectItem)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Level Select",
                    TextStyle {
                        font: menu_font.clone(),
                        font_size: 50.,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::Center),
            );
            for level_number in 0..file_amount {
                let level_name = &file_paths[level_number];
                spawn_button(
                    parent,
                    ButtonType::Level(level_number + 1),
                    menu_font.clone(),
                    &level_name[..&level_name.len() - 4],
                );
            }
            spawn_button(parent, ButtonType::Back, menu_font.clone(), "back");
        });
    level_names.0 = file_paths;
}
