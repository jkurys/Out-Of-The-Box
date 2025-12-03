use bevy::color::palettes::css::DARK_GRAY;
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
        .spawn((
            BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.3)),
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
        ))
        .insert(LevelText)
        .with_children(|parent| {
            parent.spawn((
                Text::new(format!("Level {}", current_level)),
                TextFont {
                    font_size: 30.0,
                    font: level_font.clone(),
                    ..default()
                },
                TextColor(DARK_GRAY.into()),
            ));
            parent.spawn((
                Text::new(r#"Press 'R' to restart"#),
                TextFont {
                    font_size: 20.0,
                    font: level_font.clone(),
                    ..default()
                },
                TextColor(DARK_GRAY.into()),
            ));
            parent.spawn((
                Text::new(r#"Press 'U' to undo"#),
                TextFont {
                    font_size: 20.0,
                    font: level_font.clone(),
                    ..default()
                },
                TextColor(DARK_GRAY.into()),
            ));
            parent.spawn((
                Text::new(r#"Press 'Esc' to exit the level"#),
                TextFont {
                    font_size: 20.0,
                    font: level_font.clone(),
                    ..default()
                },
                TextColor(DARK_GRAY.into()),
            ));
        });
}
