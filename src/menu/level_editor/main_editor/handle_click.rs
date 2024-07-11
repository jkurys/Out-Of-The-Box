use bevy::{prelude::*, utils::HashSet, window::PrimaryWindow};

use crate::{
    board::Board,
    components::GameEntity,
    consts::TILE_SIZE,
    game::game_objects::{Block, Direction, Floor, GameObject, Position},
};

fn offset_coordinate(coord: f32, window_coord: f32) -> i32 {
    ((coord - ((window_coord) / 2.)) / TILE_SIZE).round() as i32
}

fn vec2_to_position(vec: Vec2, window_size: Vec2) -> Position {
    Position {
        x: offset_coordinate(vec.x, window_size.x),
        y: offset_coordinate(window_size.y - vec.y, window_size.y),
    }
}

fn get_position_from_mouse_click(
    mouse: &Res<Input<MouseButton>>,
    windows: &Query<&Window, With<PrimaryWindow>>,
) -> (Position, Option<MouseButton>) {
    let window = windows.single();
    if let Some(position) = window.cursor_position() {
        let pos = vec2_to_position(
            position,
            Vec2 {
                x: window.width(),
                y: window.height(),
            },
        );
        if mouse.just_pressed(MouseButton::Left) {
            return (pos, Some(MouseButton::Left));
        }
        if mouse.just_pressed(MouseButton::Right) {
            return (pos, Some(MouseButton::Right));
        }
        return (pos, None);
    }
    (Position { x: 0, y: 0 }, None)
}

pub fn handle_level_editor_click(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut clickable_query: Query<(&Interaction, &GameEntity, &mut BackgroundColor)>,
    mouse: Res<Input<MouseButton>>,
    mut board: ResMut<Board>,
    mut entity: Local<GameEntity>,
    input: Res<Input<KeyCode>>,
    mut block_positions: Local<Option<HashSet<Position>>>,
) {
    if block_positions.is_some() {
        if input.just_pressed(KeyCode::C) {
            board.insert_block(Block {
                positions: block_positions.clone().unwrap(),
            });
            *block_positions = None;
            return;
        }
        if let (pos, Some(MouseButton::Left)) = get_position_from_mouse_click(&mouse, &windows) {
            let mut positions = block_positions.clone().unwrap();
            positions.insert(pos);
            *block_positions = Some(positions);
        }

        return;
    }

    if input.just_pressed(KeyCode::C) {
        *block_positions = Some(HashSet::new());
    }
    if let (pos, Some(button)) = get_position_from_mouse_click(&mouse, &windows) {
        if button == MouseButton::Left {
            board.insert(pos, *entity);
            if let GameEntity::Object(GameObject::HidingWall { color }) = *entity {
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
    for (&interaction, object_or_floor, mut color) in clickable_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                *entity = *object_or_floor;
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
    }) = *entity
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
            *entity = GameEntity::Object(GameObject::Turtle {
                direction: dir,
                color,
            });
        }
    }
}
