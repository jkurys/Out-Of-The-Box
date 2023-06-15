use bevy::prelude::*;

use crate::consts::*;
use crate::game::game_objects::*;
use crate::resources::Images;

use crate::board::Board;
use crate::game::movement::resources::AnimationTimer;
use crate::utils::offset_coordinate;

use super::render_2_5_d::render_object;
use super::render_entity;

//render the entire map based on Board
pub fn render_board(
    mut commands: Commands,
    mut board: ResMut<Board>,
    images: Res<Images>,
    timer: ResMut<AnimationTimer>,
) {
    if !timer.0.finished() && timer.0.elapsed_secs() != 0. {
        return;
    }
    let map_size = board.get_map_size();
    let bottom_border = offset_coordinate(0, map_size.height as i32);
    let top_border = offset_coordinate(map_size.height as i32 - 1, map_size.height as i32);
    let left_border = offset_coordinate(0, map_size.width as i32);
    let right_border = offset_coordinate(map_size.width as i32 - 1, map_size.width as i32);
    // render all objects found in board
    for y in bottom_border..(top_border + 1) {
        for x in left_border..(right_border + 1) {
            let position = Position { x, y };
            let game_object = board.get_object_type(position);
            match game_object {
                GameObject::Box => {
                    let [lower_image, higher_image] =
                        if board.get_floor_type(position) == Floor::Goal {
                            images.box_on_goal_images.clone()
                        } else {
                            images.box_images.clone()
                        };
                    let entities = render_object(
                        &mut commands,
                        higher_image,
                        lower_image,
                        position.x,
                        position.y,
                        Box,
                    );
                    board.insert_entities(position, entities);
                }
                GameObject::Wall => {
                    let [lower_image, higher_image] = images.wall_images.clone();
                    render_object(
                        &mut commands,
                        higher_image,
                        lower_image,
                        position.x,
                        position.y,
                        Wall,
                    );
                }
                GameObject::Player => {
                    let [lower_image, higher_image] = images.player_images.clone();
                    let entities =
                        render_object(&mut commands, higher_image, lower_image, x, y, Player);
                    board.insert_entities(position, entities);
                }
                GameObject::HidingWall { color } => {
                    let [lower_image, higher_image] =
                        images.shown_hidden_wall_images[color].clone();
                    let entities =
                        render_object(&mut commands, higher_image, lower_image, x, y, HiddenWall);
                    board.insert_entities(position, entities);
                }
                _ => (),
            }
        }
    }
    for y in bottom_border..(top_border + 1) {
        for x in left_border..(right_border + 1) {
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
                    render_entity(
                        HiddenWall,
                        &mut commands,
                        images.hidden_wall_images[color].clone(),
                        position,
                        FLOOR_Z_INDEX,
                    );
                }
                Floor::Button(color) => {
                    render_entity(
                        BoxButton,
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
    let [lower_wall_image, higher_wall_image] = images.wall_images.clone();
    let bottom_border = offset_coordinate(-1, map_size.height as i32);
    let top_border = offset_coordinate(map_size.height as i32, map_size.height as i32);
    let left_border = offset_coordinate(-1, map_size.width as i32);
    let right_border = offset_coordinate(map_size.width as i32, map_size.width as i32);
    //spawn horizontal border for the level and insert it to board
    for x in left_border..(right_border + 1) {
        render_object(
            &mut commands,
            higher_wall_image.clone(),
            lower_wall_image.clone(),
            x,
            top_border,
            Wall,
        );
        board.insert_object(Position { x, y: top_border }, GameObject::Wall);
    }
    for y in (bottom_border..=top_border).rev() {
        render_object(
            &mut commands,
            higher_wall_image.clone(),
            lower_wall_image.clone(),
            left_border,
            y,
            Wall,
        );
        render_object(
            &mut commands,
            higher_wall_image.clone(),
            lower_wall_image.clone(),
            right_border,
            y,
            Wall,
        );
        board.insert_object(Position { x: left_border, y }, GameObject::Wall);
        board.insert_object(Position { x: right_border, y }, GameObject::Wall);
    }
    //spawn vertical borders for the level and insert it to board
    for x in left_border..(right_border + 1) {
        render_object(
            &mut commands,
            higher_wall_image.clone(),
            lower_wall_image.clone(),
            x,
            bottom_border,
            Wall,
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
