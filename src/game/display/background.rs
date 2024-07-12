use bevy::prelude::*;

use crate::consts::*;
use crate::game::game_objects::{Direction, Button, *};
use crate::resources::{CurrentSprite, Images};

use crate::board::Board;
use crate::utils::offset_coordinate;

use super::render_2_5_d::{render_object, render_object_with_sticker, render_sticker};
use super::{render_entity, spawn_from_atlas};

fn are_both_turtles(obj1: GameObject, obj2: GameObject) -> bool {
    (matches!(
        obj1,
        GameObject::Turtle {
            direction: _,
            color: _
        } | GameObject::TurtleHead {
            direction: _,
            color: _
        }
    ) && matches!(
        obj2,
        GameObject::TurtleHead {
            direction: _,
            color: _
        }
    )) || (matches!(
        obj2,
        GameObject::Turtle {
            direction: _,
            color: _
        } | GameObject::TurtleHead {
            direction: _,
            color: _
        }
    ) && matches!(
        obj1,
        GameObject::TurtleHead {
            direction: _,
            color: _
        }
    ))
}

fn should_spawn_corner(objects: [(GameObject, Position); 4], block: &Block) -> bool {
    for (obj, pos) in objects.into_iter() {
        if !block.contains_position(pos) || obj != GameObject::Box {
            return false;
        }
    }
    true
}

//render the entire map based on Board
pub fn render_board(
    mut commands: Commands,
    mut board: ResMut<Board>,
    images: Res<Images>,
    current_sprite: Res<CurrentSprite>,
) {
    let map_size = board.get_map_size();
    let bottom_border = offset_coordinate(0, map_size.height as i32);
    let top_border = offset_coordinate(map_size.height as i32 - 1, map_size.height as i32);
    let left_border = offset_coordinate(0, map_size.width as i32);
    let right_border = offset_coordinate(map_size.width as i32 - 1, map_size.width as i32);
    // render all objects found in board
    for y in bottom_border..=top_border {
        for x in left_border..=right_border {
            let position = Position { x, y };
            let game_object = board.get_object_type(position);
            match game_object {
                GameObject::Box => {
                    let top_index = if board.get_floor_type(position) == Floor::Goal {
                        2
                    } else {
                        0
                    };

                    let [entity1, entity2] = render_object(
                        &mut commands,
                        images.box_images.clone().unwrap(),
                        (1, top_index),
                        x,
                        y,
                        Box,
                    );
                    board.insert_entities(position, [vec![entity1], vec![entity2]]);
                }
                GameObject::Wall => {
                    let [entity1, entity2] = render_object(
                        &mut commands,
                        images.wall_images.clone().unwrap(),
                        (1, 0),
                        x,
                        y,
                        Wall,
                    );
                    board.insert_entities(position, [vec![entity1], vec![entity2]]);
                }
                GameObject::Player => {
                    let [entity1, entity2] = render_object(
                        &mut commands,
                        images.player_images.clone().unwrap(),
                        (current_sprite.0 * 2 + 1, current_sprite.0 * 2),
                        x,
                        y,
                        Player,
                    );
                    board.insert_entities(position, [vec![entity1], vec![entity2]]);
                }
                GameObject::HidingWall { color } => {
                    let [entity1, entity2] = render_object(
                        &mut commands,
                        images.hidden_wall_images.clone().unwrap(),
                        (color * 3 + 1, color * 3),
                        x,
                        y,
                        HiddenWall,
                    );
                    board.insert_entities(position, [vec![entity1], vec![entity2]]);
                }
                GameObject::Turtle { color, direction } => {
                    let [entity1, entity2, entity3] = render_object_with_sticker(
                        &mut commands,
                        images.turtle_images.clone().unwrap(),
                        (direction.to_num() * 4 + 1, direction.to_num() * 4),
                        4 * 4 + color,
                        x,
                        y,
                        Turtle,
                    );
                    board.insert_entities(position, [vec![entity1, entity3], vec![entity2]]);
                }
                GameObject::TurtleHead {
                    direction,
                    color: _,
                } => {
                    let [entity1, entity2] = render_object(
                        &mut commands,
                        images.turtle_images.clone().unwrap(),
                        (direction.to_num() * 4 + 3, direction.to_num() * 4 + 2),
                        x,
                        y,
                        Turtle,
                    );
                    board.insert_entities(position, [vec![entity1], vec![entity2]]);
                }
                _ => (),
            }
            let block = board.get_block(position);
            let right_pos = position.next_position(Direction::Right);
            let left_pos = position.next_position(Direction::Left);
            let up_pos = position.next_position(Direction::Up);
            let down_pos = position.next_position(Direction::Down);

            let upper_left_pos = left_pos.next_position(Direction::Up);
            let upper_right_pos = right_pos.next_position(Direction::Up);
            let lower_left_pos = left_pos.next_position(Direction::Down);
            let lower_right_pos = right_pos.next_position(Direction::Down);

            let object = board.get_object_type(position);
            let right_object = board.get_object_type(right_pos);
            let left_object = board.get_object_type(left_pos);
            let up_object = board.get_object_type(up_pos);
            let down_object = board.get_object_type(down_pos);

            let upper_left_obj = board.get_object_type(upper_left_pos);
            let upper_right_obj = board.get_object_type(upper_right_pos);
            let lower_left_obj = board.get_object_type(lower_left_pos);
            let lower_right_obj = board.get_object_type(lower_right_pos);
            if block.contains_position(right_pos) {
                let images_opt = if object == GameObject::Box && right_object == GameObject::Box {
                    images.box_glue_images.clone()
                } else if are_both_turtles(object, right_object) {
                    None
                } else {
                    images.glue_images.clone()
                };
                if let Some(images) = images_opt {
                    let glue_entity = render_sticker(
                        &mut commands,
                        0,
                        x,
                        y,
                        images,
                        Glue,
                        UPPER_HALF_STICKER_Z_INDEX,
                    );
                    board.append_entities(position, [vec![glue_entity], Vec::new()]);
                }
            }
            if block.contains_position(left_pos) {
                let images_opt = if board.get_object_type(position) == GameObject::Box
                    && board.get_object_type(left_pos) == GameObject::Box
                {
                    images.box_glue_images.clone()
                } else if are_both_turtles(object, left_object) {
                    None
                } else {
                    images.glue_images.clone()
                };
                if let Some(images) = images_opt {
                    let glue_entity = render_sticker(
                        &mut commands,
                        1,
                        x,
                        y,
                        images,
                        Glue,
                        UPPER_HALF_STICKER_Z_INDEX,
                    );
                    board.append_entities(position, [vec![glue_entity], Vec::new()]);
                }
            }
            if block.contains_position(up_pos) {
                let images_opt = if board.get_object_type(position) == GameObject::Box
                    && board.get_object_type(up_pos) == GameObject::Box
                {
                    images.box_glue_images.clone()
                } else if are_both_turtles(object, up_object) {
                    None
                } else {
                    images.glue_images.clone()
                };
                if let Some(images) = images_opt {
                    let glue_entity = render_sticker(
                        &mut commands,
                        2,
                        x,
                        y,
                        images,
                        Glue,
                        UPPER_HALF_STICKER_Z_INDEX,
                    );
                    board.append_entities(position, [vec![glue_entity], Vec::new()]);
                }
            }
            if block.contains_position(down_pos) {
                let images_opt = if board.get_object_type(position) == GameObject::Box
                    && board.get_object_type(down_pos) == GameObject::Box
                {
                    images.box_glue_images.clone()
                } else if are_both_turtles(object, down_object) {
                    None
                } else {
                    images.glue_images.clone()
                };
                if let Some(images) = images_opt {
                    let glue_entity = render_sticker(
                        &mut commands,
                        3,
                        x,
                        y,
                        images,
                        Glue,
                        UPPER_HALF_STICKER_Z_INDEX,
                    );
                    board.append_entities(position, [vec![glue_entity], Vec::new()]);
                }
            }
            if should_spawn_corner(
                [
                    (object, position),
                    (left_object, left_pos),
                    (up_object, up_pos),
                    (upper_left_obj, upper_left_pos),
                ],
                &block,
            ) {
                let glue_entity = render_sticker(
                    &mut commands,
                    5,
                    x,
                    y,
                    images.box_glue_images.clone().unwrap(),
                    Glue,
                    CORNER_STICKER_Z_INDEX,
                );
                board.append_entities(position, [vec![glue_entity], Vec::new()]);
            }
            if should_spawn_corner(
                [
                    (object, position),
                    (right_object, right_pos),
                    (up_object, up_pos),
                    (upper_right_obj, upper_right_pos),
                ],
                &block,
            ) {
                let glue_entity = render_sticker(
                    &mut commands,
                    4,
                    x,
                    y,
                    images.box_glue_images.clone().unwrap(),
                    Glue,
                    CORNER_STICKER_Z_INDEX,
                );
                board.append_entities(position, [vec![glue_entity], Vec::new()]);
            }
            if should_spawn_corner(
                [
                    (object, position),
                    (left_object, left_pos),
                    (down_object, down_pos),
                    (lower_left_obj, lower_left_pos),
                ],
                &block,
            ) {
                let glue_entity = render_sticker(
                    &mut commands,
                    7,
                    x,
                    y,
                    images.box_glue_images.clone().unwrap(),
                    Glue,
                    CORNER_STICKER_Z_INDEX,
                );
                board.append_entities(position, [vec![glue_entity], Vec::new()]);
            }
            if should_spawn_corner(
                [
                    (object, position),
                    (right_object, right_pos),
                    (down_object, down_pos),
                    (lower_right_obj, lower_right_pos),
                ],
                &block,
            ) {
                let glue_entity = render_sticker(
                    &mut commands,
                    6,
                    x,
                    y,
                    images.box_glue_images.clone().unwrap(),
                    Glue,
                    CORNER_STICKER_Z_INDEX,
                );
                board.append_entities(position, [vec![glue_entity], Vec::new()]);
            }
        }
    }
    for y in bottom_border..=top_border {
        for x in left_border..=right_border {
            let position = Position { x, y };
            let floor = board.get_floor_type(position);
            match floor {
                Floor::Ice => {
                    render_entity(
                        Ice,
                        &mut commands,
                        images.ice_image.clone(),
                        position,
                        FLOOR_Z_INDEX,
                    );
                }
                Floor::Tile => {
                    render_entity(
                        Background,
                        &mut commands,
                        images.tile_image.clone(),
                        position,
                        FLOOR_Z_INDEX,
                    );
                }
                Floor::Goal => {
                    render_entity(
                        Goal,
                        &mut commands,
                        images.tile_image.clone(),
                        position,
                        FLOOR_Z_INDEX,
                    );
                    render_entity(
                        Goal,
                        &mut commands,
                        images.goal_image.clone(),
                        position,
                        FLOOR_STICKER_Z_INDEX,
                    );
                }
                Floor::Warp(_) => {
                    render_entity(
                        Warp,
                        &mut commands,
                        images.warp_image.clone(),
                        position,
                        FLOOR_Z_INDEX,
                    );
                }
                Floor::HiddenWall {
                    hidden_by_default: _,
                    color,
                } => {
                    spawn_from_atlas(
                        &mut commands,
                        images.hidden_wall_images.clone().unwrap(),
                        color * 3 + 2,
                        position.x,
                        position.y,
                        HiddenWall,
                    );
                }
                Floor::Button(color) => {
                    render_entity(
                        Button,
                        &mut commands,
                        images.button_images[color].clone(),
                        position,
                        FLOOR_Z_INDEX,
                    );
                }
            }
        }
    }
}

pub fn render_border(mut commands: Commands, mut board: ResMut<Board>, images: Res<Images>) {
    let map_size = board.get_map_size();
    let map = board.get_current_map();
    let bottom_border = offset_coordinate(-1, map_size.height as i32);
    let top_border = offset_coordinate(map_size.height as i32, map_size.height as i32);
    let left_border = offset_coordinate(-1, map_size.width as i32);
    let right_border = offset_coordinate(map_size.width as i32, map_size.width as i32);
    //spawn horizontal border for the level and insert it to board
    for x in left_border..=right_border {
        render_object(
            &mut commands,
            images.wall_images.clone().unwrap(),
            (1, 0),
            x,
            top_border,
            Wall,
        );
        board.insert_object_to_map_unchecked(Position { x, y: top_border }, GameObject::Wall, map);
    }
    for y in (bottom_border..=top_border).rev() {
        render_object(
            &mut commands,
            images.wall_images.clone().unwrap(),
            (1, 0),
            left_border,
            y,
            Wall,
        );
        render_object(
            &mut commands,
            images.wall_images.clone().unwrap(),
            (1, 0),
            right_border,
            y,
            Wall,
        );
        board.insert_object_to_map_unchecked(Position { x: left_border, y }, GameObject::Wall, map);
        board.insert_object_to_map_unchecked(
            Position { x: right_border, y },
            GameObject::Wall,
            map,
        );
    }
    //spawn vertical borders for the level and insert it to board
    for x in left_border..=right_border {
        render_object(
            &mut commands,
            images.wall_images.clone().unwrap(),
            (1, 0),
            x,
            bottom_border,
            Wall,
        );
        board.insert_object_to_map_unchecked(
            Position {
                x,
                y: bottom_border,
            },
            GameObject::Wall,
            map,
        );
        board.insert_object(
            Position {
                x,
                y: bottom_border,
            },
            GameObject::Wall,
        );
    }
}
