use bevy::prelude::*;

use crate::game::game_objects::*;
use crate::resources::{CurrentSprite, Images};

use crate::board::Board;
use crate::utils::offset_coordinate;
use super::glue::render_glue;
use super::floor::render_floor;
use super::render_2_5_d::{render_object, render_object_with_sticker};

pub fn calculate_borders(board: &ResMut<Board>) -> (i32, i32, i32, i32) {
    let map_size = board.get_map_size();
    let bottom_border = offset_coordinate(0, map_size.height as i32);
    let top_border = offset_coordinate(map_size.height as i32 - 1, map_size.height as i32);
    let left_border = offset_coordinate(0, map_size.width as i32);
    let right_border = offset_coordinate(map_size.width as i32 - 1, map_size.width as i32);
    (bottom_border, top_border, left_border, right_border)
}

//render the entire map based on Board
pub fn render_board(
    mut commands: Commands,
    mut board: ResMut<Board>,
    images: Res<Images>,
    current_sprite: Res<CurrentSprite>,
) {
    let (bottom_border, top_border, left_border, right_border) = calculate_borders(&board);
    // render all objects found in board
    for y in bottom_border..=top_border {
        for x in left_border..=right_border {
            let z = 1;
            let position = Position { x, y, z };
            let game_object = board.get_object_type(position);
            match game_object {
                GameObject::Box => {
                    let top_index = if board.get_floor_type(position.position_below()) == Floor::Goal {
                        3
                    } else {
                        0
                    };

                    let [entity1, entity2, entity3] = render_object(
                        &mut commands,
                        images.box_images.clone().unwrap(),
                        (1, top_index, 4),
                        x,
                        y,
                        z,
                        0.,
                        Box,
                    );
                    board.insert_entities(position, [vec![entity1, entity3], vec![entity2]]);
                }
                GameObject::Wall => {
                    let [entity1, entity2, entity3] = render_object(
                        &mut commands,
                        images.wall_images.clone().unwrap(),
                        (1, 0, 2),
                        x,
                        y,
                        z,
                        0.,
                        Wall,
                    );
                    board.insert_entities(position, [vec![entity1, entity3], vec![entity2]]);
                }
                GameObject::Player => {
                    let counter = board.get_eat_counter(position);
                    if counter.is_none() {
                        let [entity1, entity2, entity3] = render_object(
                            &mut commands,
                            images.player_images.clone().unwrap(),
                            (current_sprite.0 * 4 + 1, current_sprite.0 * 4, current_sprite.0 * 4 + 2),
                            x,
                            y,
                            z,
                            0.,
                            Player,
                        );
                        board.insert_entities(position, [vec![entity1, entity3], vec![entity2]]);
                    } else {
                        let mut counter = counter.unwrap();
                        if counter == 0 {
                            counter = 1;
                        }
                        let [entity1, entity2, entity3, entity4] = render_object_with_sticker(
                            &mut commands,
                            images.player_images.clone().unwrap(),
                            (current_sprite.0 * 4 + 1, current_sprite.0 * 4, current_sprite.0 * 4 + 2),
                            16 - counter,
                            x,
                            y,
                            z,
                            0.,
                            Player,
                        );
                        board.insert_entities(position, [vec![entity1, entity3, entity4], vec![entity2]]);
                    }
                }
                GameObject::HidingWall { color } => {
                    let [entity1, entity2, entity3] = render_object(
                        &mut commands,
                        images.hidden_wall_images.clone().unwrap(),
                        (color * 3 + 1, color * 3, color * 3 + 2),
                        x,
                        y,
                        z,
                        0.,
                        HiddenWall,
                    );
                    board.insert_entities(position, [vec![entity1, entity3], vec![entity2]]);
                }
                GameObject::Turtle { color, direction } => {
                    let [entity1, entity2, entity3, entity4] = render_object_with_sticker(
                        &mut commands,
                        images.turtle_images.clone().unwrap(),
                        (direction.to_num() * 6 + 1, direction.to_num() * 6, direction.to_num() * 6 + 2),
                        4 * 6 + color,
                        x,
                        y,
                        z,
                        0.,
                        Turtle,
                    );
                    board.insert_entities(position, [vec![entity1, entity3, entity4], vec![entity2]]);
                }
                GameObject::TurtleHead {
                    direction,
                    color: _,
                } => {
                    let [entity1, entity2, entity3] = render_object(
                        &mut commands,
                        images.turtle_images.clone().unwrap(),
                        (direction.to_num() * 6 + 4, direction.to_num() * 6 + 3, direction.to_num() * 6 + 5),
                        x,
                        y,
                        z,
                        0.,
                        Turtle,
                    );
                    board.insert_entities(position, [vec![entity1, entity3], vec![entity2]]);
                }
                _ => (),
            }
            render_glue(position, &mut board, &images, &mut commands);
        }
    }
    render_floor(commands, board, images, current_sprite);
}
