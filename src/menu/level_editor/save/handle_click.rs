use bevy::prelude::*;

use super::{LevelEditorFileName, events::FileSavedEvent};

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