use std::{fs::File, io::Write};

use bevy::prelude::*;

use crate::{state::DisplayState, consts::MAIN_MENU_FONT, resources::StateStack, board::Board};

use super::events::FileSavedEvent;

#[derive(Component)]
pub struct LevelEditorSaveItem;

#[derive(Component)]
pub struct LevelEditorFileName;

pub fn handle_exit_to_save(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<NextState<DisplayState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_state.set(DisplayState::LevelEditorSave);
        keyboard_input.reset(KeyCode::Escape);
    }
}

pub fn setup_file_name_getter(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
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
        .insert(LevelEditorSaveItem)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Please provide the level name",
                TextStyle {
                    font: menu_font.clone(),
                    font_size: 50.,
                    color: Color::WHITE,
                },
            ));
            parent.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font: menu_font.clone(),
                    font_size: 30.,
                    color: Color::WHITE,
                },
            )).insert(LevelEditorFileName);
        });
}

pub fn handle_file_get(
    mut char_reader: EventReader<ReceivedCharacter>,
    input: ResMut<Input<KeyCode>>,
    mut file_name: Local<String>,
    mut event_writer: EventWriter<FileSavedEvent>,
    mut change_name: Query<&mut Text, With<LevelEditorFileName>>,
) {
    for ev in char_reader.iter() {
        if ev.char.is_ascii_alphanumeric() {
            let mut text = change_name.single_mut();
            text.sections[0].value.push(ev.char);
            file_name.push(ev.char);
        }
    }
    if input.just_pressed(KeyCode::Return) {
        event_writer.send(FileSavedEvent(file_name.clone()));
        *file_name = "".to_string();
    }
    if input.just_pressed(KeyCode::Back) {
        let mut text = change_name.single_mut();
        text.sections[0].value.pop();
        file_name.pop();
    }
}

pub fn save_board_to_file(
    mut board: ResMut<Board>,
    mut reader: EventReader<FileSavedEvent>,
    mut app_state: ResMut<NextState<DisplayState>>,
    mut state_stack: ResMut<StateStack>,
) {
    let mut file_name = "".to_string();
    for ev in reader.iter() {
        file_name = ev.0.clone();
    }
    if file_name == "".to_string() {
        return;
    }
    let mut file = File::create(format!("assets/maps/{}.txt", file_name)).unwrap();
    let file_prelude = serde_json::to_string(&board.clone());
    match file_prelude {
        Ok (str) => {
            let buf = str.chars().map(|c| c as u8).collect::<Vec<_>>();
            file.write_all(&buf[..]).unwrap();
        }
        _ => ()
    }
    app_state.set(state_stack.0.pop().unwrap_or(DisplayState::MainMenu));
    board.clear();
}