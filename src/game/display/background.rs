use bevy::prelude::*;

use crate::consts::*;
use crate::game::game_objects::{
    Player,
    HiddenWall,
    Turtle,
    Button,
    Background,
    Position,
    Wall,
    Box,
    GameObject,
    Floor,
    Ice,
    Goal,
    Warp
};
use crate::resources::{CurrentSprite, Images};

use crate::board::Board;
use crate::utils::offset_coordinate;

use super::render_2_5_d::{render_object, render_object_with_sticker};
use super::{render_entity, spawn_from_atlas};

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
