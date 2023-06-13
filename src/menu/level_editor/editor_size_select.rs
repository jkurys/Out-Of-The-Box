use bevy::prelude::*;

use crate::{consts::{MAIN_MENU_FONT, MAX_WIDTH, MAX_HEIGHT}, state::DisplayState};

use super::{editor::{LevelEditorItem, LevelEditorInputNumber, LevelEditorStartingPrompt}, resources::BoardSize};

pub fn setup_level_editor(asset_server: Res<AssetServer>, mut commands: Commands) {
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
        .insert(LevelEditorItem)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Please provide the level width",
                TextStyle {
                    font: menu_font.clone(),
                    font_size: 50.,
                    color: Color::WHITE,
                },
            )).insert(LevelEditorStartingPrompt);
            parent.spawn(TextBundle::from_section(
                "0",
                TextStyle {
                    font: menu_font.clone(),
                    font_size: 50.,
                    color: Color::WHITE,
                },
            )).insert(LevelEditorInputNumber);
        });
}

pub fn handle_level_editor_input(
    mut char_reader: EventReader<ReceivedCharacter>,
    mut input: ResMut<Input<KeyCode>>,
    mut width: Local<u32>,
    mut height: Local<u32>,
    mut is_width_provided: Local<bool>,
    mut app_state: ResMut<NextState<DisplayState>>,
    mut board_size: ResMut<BoardSize>,
    mut change_prompt: Query<(&mut Text, (With<LevelEditorStartingPrompt>, Without<LevelEditorInputNumber>))>,
    mut change_number: Query<(&mut Text, (With<LevelEditorInputNumber>, Without<LevelEditorStartingPrompt>))>,
) {
    for ev in char_reader.iter() {
        if ev.char.is_ascii_digit() && !*is_width_provided {
            *width = *width * 10 + ev.char.to_digit(10).unwrap();
            if *width > MAX_WIDTH {
                *width = 0;
            }
            let (mut text, _) = change_number.single_mut();
            text.sections[0].value = width.to_string();
        } else if ev.char.is_ascii_digit() {
            *height = *height * 10 + ev.char.to_digit(10).unwrap();
            if *height > MAX_HEIGHT {
                *height = 0;
            }
            let (mut text, _) = change_number.single_mut();
            text.sections[0].value = height.to_string();
        }
    }
    if input.just_pressed(KeyCode::Return) && !*is_width_provided {
        *is_width_provided = true;
        input.reset(KeyCode::Return);
        let (mut text, _) = change_prompt.single_mut();
        text.sections[0].value = "Please provide the level height".to_string();
        let (mut text2, _) = change_number.single_mut();
        text2.sections[0].value = 0.to_string();
    }
    if input.just_pressed(KeyCode::Return) && *is_width_provided {
        *is_width_provided = false;
        *board_size = BoardSize { width: *width, height: *height };
        app_state.set(DisplayState::LevelEditorBoard);
        *height = 0;
        *width = 0;
    }
}