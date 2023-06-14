use std::{fs::File, io::Write};

use bevy::prelude::*;

use crate::{state::DisplayState, consts::MAIN_MENU_FONT, game::game_objects::{Position, GameObject, Floor}, resources::{StateStack, Board, MapSize}};

use super::{events::FileSavedEvent};

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

fn position_to_index(pos: Position, width: u32, height: u32) -> usize {
    let x = pos.x + (width / 2) as i32;
    let y = pos.y + (height / 2) as i32;
    x as usize
        + y as usize * (width + 1) as usize
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
    let maps_char = char::from_digit(board.get_created_maps() as u32, 10).unwrap();
    let mut file_prelude = vec![maps_char, '\n'];

    for n in 0..board.get_created_maps() {
        let (objects, floors) = board.get_board_n(n);
        let MapSize { width, height } = board.get_map_size_n(n);
        let mut height_string: Vec<char> = height.to_string().chars().collect();
        let mut width_string: Vec<char> = width.to_string().chars().collect();
        let mut map_prelude = Vec::new();
        map_prelude.append(&mut height_string);
        map_prelude.append(&mut vec![' ']);
        map_prelude.append(&mut width_string);
        map_prelude.append(&mut vec!['\n']);
        let mut buf = vec![' '; (width + 1) as usize * height as usize];
        for i in 0..height {
            buf[(width + i * (width + 1)) as usize] = '\n';
        }
        for (position, object) in objects.iter() {
            let index = position_to_index(*position, width, height);
            buf[index] = match *object {
                GameObject::Box => 'b',
                GameObject::Wall => 'w',
                GameObject::HidingWall { color: 0 } => 'H',
                GameObject::HidingWall { color: 1 } => 'J',
                GameObject::HidingWall { color: _ } => 'K',
                GameObject::Empty => ' ',
                GameObject::Player => 'p',
            };
            for (position, floor) in floors.iter() {
                let index = position_to_index(*position, width, height);
                buf[index] = match *floor {
                    Floor::HiddenWall {
                        hidden_by_default: _,
                        color: 0,
                    } => 'h',
                    Floor::HiddenWall {
                        hidden_by_default: _,
                        color: 1,
                    } => 'j',
                    Floor::HiddenWall {
                        hidden_by_default: _,
                        color: _,
                    } => 'k',
                    Floor::Tile => ' ',
                    Floor::Ice => 'i',
                    Floor::Goal => 'g',
                    Floor::Warp(num) => char::from_digit(num as u32, 10).unwrap(),
                    Floor::Button(0) => 't',
                    Floor::Button(1) => 's',
                    Floor::Button(_) => 'u',
                };
            }
        }
        map_prelude.append(&mut buf);
        file_prelude.append(&mut map_prelude);
    }
    let mut file = File::create(format!("assets/maps/{}.txt", file_name)).unwrap();
    let buf = file_prelude.iter().map(|c| *c as u8).collect::<Vec<_>>();
    file.write_all(&buf[..]).unwrap();
    app_state.set(state_stack.0.pop().expect("Could not save file"));
    board.clear();
}