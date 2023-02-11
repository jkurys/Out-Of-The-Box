use bevy::prelude::*;

use crate::consts::*;
use crate::game::game_objects::*;
use crate::resources::Images;

use crate::game::movement::resources::AnimationTimer;
use crate::game::resources::Board;

use super::render_entity;

fn offset_coordinate(coord: i32, max: i32) -> i32 {
    coord - (max / 2)
}

//render the entire map based on Board
pub fn render_board(
    mut commands: Commands,
    mut board: ResMut<Board>,
    images: Res<Images>,
    timer: Res<AnimationTimer>,
) {
    if !timer.0.finished() && timer.0.elapsed_secs() != 0. {
        return;
    }
    let map_size = board.get_map_size();
    let bottom_border = offset_coordinate(0, map_size.height as i32);
    let top_border = offset_coordinate(map_size.height as i32 - 1, map_size.height as i32);
    let left_border = offset_coordinate(0, map_size.width as i32);
    let right_border = offset_coordinate(map_size.width as i32 - 1, map_size.width as i32);
    let map = board.get_current_map();
    // render all objects found in board
    for y in bottom_border..(top_border + 1) {
        for x in left_border..(right_border + 1) {
            let position = Position { x, y, map };
            let game_object = board.get_object_type(position);
            match game_object {
                GameObject::Box => {
                    let image = if board.get_floor_type(position) == Floor::Goal {
                        images.box_on_goal_image.clone()
                    } else {
                        images.box_image.clone()
                    };
                    let entity = render_entity(Box, &mut commands, image, position, OBJECT_Z_INDEX);
                    board.insert_entity(position, entity);
                }
                GameObject::Wall => {
                    render_entity(
                        Wall,
                        &mut commands,
                        images.wall_image.clone(),
                        position,
                        OBJECT_Z_INDEX,
                    );
                }
                GameObject::Player => {
                    let entity = render_entity(
                        Player,
                        &mut commands,
                        images.player_image.clone(),
                        position,
                        OBJECT_Z_INDEX,
                    );
                    board.insert_entity(position, entity);
                }
                GameObject::HidingWall => {
                    let entity = render_entity(
                        HiddenWall,
                        &mut commands,
                        images.shown_hidden_wall_image.clone(),
                        position,
                        OBJECT_Z_INDEX,
                    );
                    board.insert_entity(position, entity);
                }
                _ => (),
            }
        }
    }
    for y in bottom_border..(top_border + 1) {
        for x in left_border..(right_border + 1) {
            let position = Position { x, y, map };
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
                        images.goal_image.clone(),
                        position,
                        FLOOR_Z_INDEX,
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
                Floor::HiddenWall{hidden_by_default: _} => {
                    render_entity(
                        HiddenWall,
                        &mut commands,
                        images.hidden_wall_image.clone(),
                        position,
                        FLOOR_Z_INDEX,
                    );
                }
                Floor::Button => {
                    render_entity(BoxButton, &mut commands, images.button_image.clone(), position, FLOOR_Z_INDEX);
                }
            }
        }
    }
}

pub fn render_border(
    mut commands: Commands,
    mut board: ResMut<Board>,
    images: Res<Images>,
    timer: Res<AnimationTimer>,
) {
    if !timer.0.finished() && timer.0.elapsed_secs() != 0. {
        return;
    }
    let map_size = board.get_map_size();
    let bottom_border = offset_coordinate(-1, map_size.height as i32);
    let top_border = offset_coordinate(map_size.height as i32, map_size.height as i32);
    let left_border = offset_coordinate(-1, map_size.width as i32);
    let right_border = offset_coordinate(map_size.width as i32, map_size.width as i32);
    let map = board.get_current_map();
    //spawn horizontal border for the level and insert it to board
    for y in bottom_border..(top_border + 1) {
        render_entity(
            Wall,
            &mut commands,
            images.wall_image.clone(),
            Position {
                x: left_border,
                y,
                map,
            },
            OBJECT_Z_INDEX,
        );
        render_entity(
            Wall,
            &mut commands,
            images.wall_image.clone(),
            Position {
                x: right_border,
                y,
                map,
            },
            OBJECT_Z_INDEX,
        );
        board.insert_object(
            Position {
                x: left_border,
                y,
                map,
            },
            GameObject::Wall,
        );
        board.insert_object(
            Position {
                x: right_border,
                y,
                map,
            },
            GameObject::Wall,
        );
    }
    //spawn vertical borders for the level and insert it to board
    for x in left_border..(right_border + 1) {
        render_entity(
            Wall,
            &mut commands,
            images.wall_image.clone(),
            Position {
                x,
                y: top_border,
                map,
            },
            OBJECT_Z_INDEX,
        );
        render_entity(
            Wall,
            &mut commands,
            images.wall_image.clone(),
            Position {
                x,
                y: bottom_border,
                map,
            },
            OBJECT_Z_INDEX,
        );
        board.insert_object(
            Position {
                x,
                y: top_border,
                map,
            },
            GameObject::Wall,
        );
        board.insert_object(
            Position {
                x,
                y: bottom_border,
                map,
            },
            GameObject::Wall,
        );
    }
}
