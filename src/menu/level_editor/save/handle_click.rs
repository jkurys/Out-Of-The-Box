use bevy::{prelude::*, input::keyboard::{KeyboardInput, Key}};

use super::{events::FileSavedEvent, LevelEditorFileName};

pub fn handle_file_get(
    mut char_reader: EventReader<KeyboardInput>,
    input: ResMut<ButtonInput<KeyCode>>,
    mut file_name: Local<String>,
    mut event_writer: EventWriter<FileSavedEvent>,
    mut change_name: Query<&mut Text, With<LevelEditorFileName>>,
) {
    for ev in char_reader.read() {
        if !ev.state.is_pressed() {
            continue;
        }
        match &ev.logical_key {
            Key::Character(character) => {
                let c = character.chars().last().unwrap();
                if c.is_ascii_alphanumeric() {
                    let mut text = change_name.single_mut();
                    text.sections[0].value.push(c);
                    file_name.push(c);
                }
            },
            _ => (),
        }
        
    }
    if input.just_pressed(KeyCode::Enter) {
        event_writer.send(FileSavedEvent(file_name.clone()));
        *file_name = "".to_string();
    }
    if input.just_pressed(KeyCode::Backspace) {
        let mut text = change_name.single_mut();
        text.sections[0].value.pop();
        file_name.pop();
    }
}
