use bevy::{prelude::*, utils::HashSet, window::PrimaryWindow};
use bevy::color::palettes::css::WHITE;
use crate::game::display::background::calculate_borders;
use crate::game::display::render_2_5_d::get_offsets;

use crate::{
    board::Board,
    components::GameEntity,
    consts::*,
    game::game_objects::{Block, Direction, Floor, GameObject, Position},
};

fn is_inside_rect(
    mouse_pos: Vec2,
    xs: (f32, f32),
    ys: (f32, f32),
) -> bool {
    let Vec2 { x, y } = mouse_pos;
    return x > xs.0 && x < xs.1
        && y > ys.0 && y < ys.1;
}

fn mouse_over_column(
    mut zs: Vec<i32>,
    x: i32,
    y: i32,
    mouse_position: Vec2,
    window: Window,
) -> Option<Position> {
    zs.sort();
    zs.reverse();
    let mouse_position = Vec2 { x: mouse_position.x - window.width() / 2., y: window.height() / 2. - mouse_position.y };
    for &z in zs.iter() {
        let (mut uppers, mut lowers, mut sides) = get_offsets(x, y, z, 0.);
        uppers = (uppers.0 - TILE_WIDTH / 2. - 5., uppers.1 - 5., uppers.2);
        lowers = (lowers.0 - TILE_WIDTH / 2. - 13., lowers.1 + 5., lowers.2);
        sides = (sides.0 + TILE_WIDTH / 2. - SIDE_WIDTH, sides.1 - TILE_HEIGHT, sides.2);
        if is_inside_rect(mouse_position, (uppers.0, uppers.0 + TILE_WIDTH), (uppers.1, uppers.1 + TILE_HEIGHT)) {
            return Some(Position { x, y, z });
        } else if is_inside_rect(mouse_position, (lowers.0, lowers.0 + TILE_WIDTH), (lowers.1, lowers.1 + TILE_FRONT_HEIGHT)) {
            return Some(Position { x, y: y - 1, z: z - 1 });
        } else if is_inside_rect(mouse_position, (sides.0, sides.0 + SIDE_WIDTH * 2.), (sides.1, sides.1 + SIDE_HEIGHT)) {
            return Some(Position { x: x + 1, y, z: z - 1 });
        }
    }
    None
}

pub fn get_frontmost_position(
    mouse: &Res<ButtonInput<MouseButton>>,
    windows: &Query<&Window, With<PrimaryWindow>>,
    board: &ResMut<Board>,
) -> (Position, Option<MouseButton>) {
    let window = windows.single();
    if let Some(position) = window.cursor_position() {
        let (bot_border, top_border, left_border, right_border) = calculate_borders(board);
        for y in bot_border..=top_border {
            for x in left_border..=right_border {
                if let Some(pos) = mouse_over_column(
                    board.get_column(x, y),
                    x,
                    y,
                    position,
                    window.clone(),
                ) {
                    if mouse.just_pressed(MouseButton::Left) {
                        return (pos, Some(MouseButton::Left));
                    }
                    if mouse.just_pressed(MouseButton::Right) {
                        return (pos, Some(MouseButton::Right));
                    }
                    return (pos, None);
                }
            }
        }
    }
    (Position { x: 0, y: 0, z: 0 }, None)
}

pub fn handle_level_editor_click(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut clickable_query: Query<(&Interaction, &GameEntity, &mut BackgroundColor)>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut board: ResMut<Board>,
    mut entity: Local<GameEntity>,
    input: Res<ButtonInput<KeyCode>>,
    mut block_positions: Local<Option<HashSet<Position>>>,
) {
    
    let mouse_pos_opt = get_frontmost_position(&mouse, &windows, &board);
    if block_positions.is_some() {
        if input.just_pressed(KeyCode::KeyC) {
            board.insert_block(Block {
                positions: block_positions.clone().unwrap(),
            });
            *block_positions = None;
            return;
        }
        if let (mut pos, Some(MouseButton::Left)) = mouse_pos_opt {
            pos = Position { x: pos.x, y: pos.y, z: pos.z };
            let mut positions = block_positions.clone().unwrap();
            positions.insert(pos);
            *block_positions = Some(positions);
        }

        return;
    }

    if input.just_pressed(KeyCode::KeyC) {
        *block_positions = Some(HashSet::new());
    }
    if let (mut pos, Some(button)) = mouse_pos_opt {
        if button == MouseButton::Left {
            if matches!(*entity, GameEntity::Object(GameObject::HidingWall { color: _, hidden_toggle: true, hidden_by_def: true })) {
                pos = Position { x: pos.x, y: pos.y, z: pos.z };
            }
            else if let GameEntity::Object(_) = *entity {
                pos = Position { x: pos.x, y: pos.y, z: pos.z + 1 };
            }
            if let GameEntity::Floor(Floor::Tile) = *entity {
                pos = Position { x: pos.x, y: pos.y, z: pos.z };
            }
            board.insert(pos, *entity);
            if let GameEntity::Object(GameObject::HidingWall { color, hidden_toggle: false, hidden_by_def: false }) = *entity {
                board.insert_floor(
                    pos.position_below(),
                    Floor::HiddenWall {
                        hidden_by_default: false,
                        color,
                    },
                );
            }
        } else if button == MouseButton::Right {
            board.delete_object(pos);
            board.delete_floor(pos);
            if pos.z == 0 {
                board.insert_object(pos, GameObject::Wall);
                board.insert_floor(pos, Floor::Tile);
            }
        }
    }
    for (&interaction, object_or_floor, mut color) in clickable_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                *entity = *object_or_floor;
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.7, 0.7, 0.7).into();
            }
            Interaction::None => {
                *color = WHITE.into();
            }
        }
    }
    if let GameEntity::Object(GameObject::Turtle {
        direction: _,
        color,
    }) = *entity
    {
        let mut maybe_dir = None;
        if input.just_pressed(KeyCode::ArrowUp) {
            maybe_dir = Some(Direction::North);
        } else if input.just_pressed(KeyCode::ArrowDown) {
            maybe_dir = Some(Direction::South);
        } else if input.just_pressed(KeyCode::ArrowLeft) {
            maybe_dir = Some(Direction::Left);
        } else if input.just_pressed(KeyCode::ArrowRight) {
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
