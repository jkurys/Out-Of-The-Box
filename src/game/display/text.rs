use bevy::prelude::*;

use crate::{consts::LEVEL_FONT, resources::CurrentLevel};

#[derive(Component)]
pub struct LevelText;

pub fn display_level_text(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    asset_server: Res<AssetServer>,
) {
    let level_font = asset_server.load(LEVEL_FONT);
    let current_level = current_level.level_number;
    commands
        .spawn(NodeBundle {
            background_color: Color::rgba(0.5, 0.5, 0.5, 0.3).into(),
            visibility: Visibility::Visible,
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        })
        .insert(LevelText)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    format!("Level {}", current_level),
                    TextStyle {
                        font_size: 30.0,
                        color: Color::DARK_GRAY,
                        font: level_font.clone(),
                    },
                )
                .with_text_justify(JustifyText::Center),
            );
            parent.spawn(
                TextBundle::from_section(
                    r#"Press 'R' to restart"#,
                    TextStyle {
                        font_size: 20.0,
                        color: Color::DARK_GRAY,
                        font: level_font.clone(),
                    },
                )
                .with_text_justify(JustifyText::Center),
            );
            parent.spawn(
                TextBundle::from_section(
                    r#"Press 'U' to undo"#,
                    TextStyle {
                        font_size: 20.0,
                        color: Color::DARK_GRAY,
                        font: level_font.clone(),
                    },
                )
                .with_text_justify(JustifyText::Center),
            );
            parent.spawn(
                TextBundle::from_section(
                    r#"Press 'Esc' to exit the level"#,
                    TextStyle {
                        font_size: 20.0,
                        color: Color::DARK_GRAY,
                        font: level_font.clone(),
                    },
                )
                .with_text_justify(JustifyText::Center),
            );
        });
}
