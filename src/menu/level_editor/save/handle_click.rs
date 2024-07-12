use bevy::prelude::*;

use super::{events::FileSavedEvent, LevelEditorFileName};

pub fn handle_file_get(
    mut char_reader: EventReader<ReceivedCharacter>,
    input: ResMut<ButtonInput<KeyCode>>,
    mut file_name: Local<String>,
    mut event_writer: EventWriter<FileSavedEvent>,
    mut change_name: Query<&mut Text, With<LevelEditorFileName>>,
) {
    for ev in char_reader.read() {
        let c = ev.char.chars().last().unwrap();
        if c.is_ascii_alphanumeric() {
            let mut text = change_name.single_mut();
            text.sections[0].value.push(c);
            file_name.push(c);
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
