use bevy::{prelude::*, utils::HashSet, window::PrimaryWindow};
use crate::game::display::render_2_5_d::render_object;

use crate::game::game_objects::Background;
use crate::{
    board::Board,
    components::GameEntity,
    consts::*,
    game::game_objects::{Block, Direction, Floor, GameObject, Position}, resources::Images,
};

fn offset_coordinate_x(coord: f32, window_coord: f32, y: i32) -> i32 {
    ((coord - ((window_coord) / 2.)) / TILE_WIDTH - (y as f32 * (101./300.))).round() as i32
}

fn offset_coordinate_y(coord: f32, window_coord: f32) -> i32 {
    ((coord - ((window_coord) / 2.)) / TILE_HEIGHT).floor() as i32
}

fn vec2_to_position(vec: Vec2, window_size: Vec2) -> Position {
    let y = offset_coordinate_y(window_size.y - vec.y, window_size.y);
    Position {
        x: offset_coordinate_x(vec.x, window_size.x, y),
        y,
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
    images: Res<Images>,
    mut commands: Commands,
) {
    let (mouse_pos, _) = get_position_from_mouse_click(&mouse, &windows);
    render_object(&mut commands, images.highlight_images.clone().unwrap(), (1, 0, 2), mouse_pos.x, mouse_pos.y, 0, 0., Background);
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
