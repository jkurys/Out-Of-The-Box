use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    board::Board,
    components::GameEntity,
    consts::{TURTLE_TEXTURES, TILE_SIZE},
    game::game_objects::{Direction, Floor, GameObject, Position},
};

use super::LevelEditorChangable;

fn offset_coordinate(
    coord: f32,
    window_coord: f32,
) -> i32 {
    ((coord - ((window_coord) / 2.)) / TILE_SIZE).round() as i32
}

fn vec2_to_position(
    vec: Vec2,
    window_size: Vec2,
) -> Position {
    Position { x: offset_coordinate(vec.x, window_size.x), y: offset_coordinate(vec.y, window_size.y) }
}

fn get_position_from_mouse_click(
    mouse: &Res<Input<MouseButton>>,
    windows: &Query<&Window, With<PrimaryWindow>>,
) -> Option<(Position, MouseButton)> {
    let window = windows.single();
    if let Some(position) = window.cursor_position() {
        let pos = vec2_to_position(position, Vec2 { x: window.width(), y: window.height() });
        if mouse.just_pressed(MouseButton::Left) {
            return Some((pos, MouseButton::Left));
        }
        if mouse.just_pressed(MouseButton::Right) {
            return Some((pos, MouseButton::Right));
        }
    }
    None
}

pub fn handle_level_editor_click(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut clickable_query: Query<
        (&Interaction, &UiImage, &GameEntity, &mut BackgroundColor),
        Without<LevelEditorChangable>,
    >,
    mouse: Res<Input<MouseButton>>,
    mut board: ResMut<Board>,
    mut current_object: Local<GameEntity>,
    mut image: Local<(UiImage, bool)>,
    input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    if let Some((pos, button)) = get_position_from_mouse_click(&mouse, &windows) {
        if button == MouseButton::Left {
            board.insert(pos, *current_object);
            if let GameEntity::Object(GameObject::HidingWall { color }) = *current_object {
                board.insert_floor(
                    pos,
                    Floor::HiddenWall {
                        hidden_by_default: false,
                        color,
                    },
                );
            }
        } else if button == MouseButton::Right {
            board.delete_object(pos);
            board.delete_floor(pos);
        }
    }
    for (&interaction, new_image, object_or_floor, mut color) in clickable_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                image.0 = new_image.clone();
                image.1 = true;
                *current_object = *object_or_floor;
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.7, 0.7, 0.7).into();
            }
            Interaction::None => {
                *color = Color::WHITE.into();
            }
        }
    }
    if let GameEntity::Object(GameObject::Turtle {
        direction: _,
        color,
    }) = *current_object
    {
        let mut maybe_dir = None;
        if input.just_pressed(KeyCode::Up) {
            maybe_dir = Some(Direction::Up);
        } else if input.just_pressed(KeyCode::Down) {
            maybe_dir = Some(Direction::Down);
        } else if input.just_pressed(KeyCode::Left) {
            maybe_dir = Some(Direction::Left);
        } else if input.just_pressed(KeyCode::Right) {
            maybe_dir = Some(Direction::Right);
        }
        if let Some(dir) = maybe_dir {
            image.0 = asset_server.load(TURTLE_TEXTURES[dir.to_num()]).into();
            *current_object = GameEntity::Object(GameObject::Turtle {
                direction: dir,
                color,
            });
        }
    }
}
