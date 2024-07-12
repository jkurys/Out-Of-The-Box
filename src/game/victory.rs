use bevy::prelude::*;

use std::fs::File;
use std::io::{Read, Write};

use super::resources::VictoryTimer;
use crate::board::Board;
use crate::consts::{LEVEL_SAVE, MAIN_MENU_FONT};
use crate::resources::{CurrentLevel, StateStack};
use crate::state::DisplayState;

use super::game_objects::GameObject;

#[derive(Component)]
pub struct VictoryItem;

pub fn handle_win(
    board: Res<Board>,
    mut display_state: ResMut<NextState<DisplayState>>,
    mut timer: ResMut<VictoryTimer>,
    time: Res<Time>,
    current_level: Res<CurrentLevel>,
) {
    let mut is_win = true;
    for position in board.get_all_goals().iter() {
        if board.get_object_type(*position) != GameObject::Box {
            is_win = false;
        }
    }
    if is_win {
        timer.0.tick(time.delta());
    } else {
        timer.0.reset();
    }
    if timer.0.finished() {
        let file_read = File::open(LEVEL_SAVE);
        let level_amount = current_level.level_amount;
        let mut buf = vec![0_u8; level_amount];
        if let Ok(mut read) = file_read {
            let result = read.read_exact(&mut buf);
            if let Ok(()) = result {
                if buf[current_level.level_number - 1] == 0 {
                    let mut file_write = File::create(LEVEL_SAVE).unwrap();
                    buf[current_level.level_number - 1] = 1;
                    file_write.write_all(&buf).unwrap();
                }
            } else {
                let mut file_write = File::create(LEVEL_SAVE).unwrap();
                let mut buf = vec![0_u8; level_amount];
                buf[current_level.level_number - 1] = 1;
                file_write.write_all(&buf).unwrap();
            }
        } else {
            let mut file_write = File::create(LEVEL_SAVE).unwrap();
            let mut buf = vec![0_u8; level_amount];
            buf[current_level.level_number - 1] = 1;
            file_write.write_all(&buf).unwrap();
        }

        display_state.set(DisplayState::Victory);
        timer.0.reset();
    }
}

pub fn setup_win(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let menu_font = asset_server.load(MAIN_MENU_FONT);
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::LIME_GREEN),
            visibility: Visibility::Visible,
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        })
        .insert(VictoryItem)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Level completed",
                    TextStyle {
                        font_size: 50.0,
                        color: Color::WHITE,
                        font: menu_font.clone(),
                    },
                )
                .with_text_justify(JustifyText::Center),
            );
            parent.spawn(
                TextBundle::from_section(
                    "Press Enter to continue",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::DARK_GREEN,
                        font: menu_font.clone(),
                    },
                )
                .with_text_justify(JustifyText::Center),
            );
        });
}

pub fn handle_win_click(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<DisplayState>>,
    mut state_stack: ResMut<StateStack>,
) {
    if keyboard_input.pressed(KeyCode::Enter) {
        app_state.set(
            state_stack
                .0
                .pop()
                .expect("Could not go out of victory screen"),
        );
        keyboard_input.reset(KeyCode::Enter);
    }
}
