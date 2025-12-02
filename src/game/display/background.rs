use bevy::prelude::*;

use crate::game::game_objects::*;
use crate::resources::{CurrentSprite, Images};

use super::floor::render_floor;
use super::glue::render_glue;
use super::render_2_5_d::{render_object, render_object_gray, render_object_with_sticker};
use crate::board::{Board, GameData};
use crate::utils::offset_coordinate;

pub fn calculate_borders(board: &Board) -> (i32, i32, i32, i32) {
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
    mut game_data: ResMut<GameData>,
    images: Res<Images>,
    current_sprite: Res<CurrentSprite>,
) {
    let objects = game_data.board.get_objects();
    for (&position, &game_object) in objects.iter() {
        let GameData {
            board,
            entity_storage,
        } = &mut *game_data;
        let Position { x, y, z } = position;
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
                entity_storage
                    .insert_entities(position, [vec![entity1], vec![entity2], vec![entity3]]);
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
                entity_storage
                    .insert_entities(position, [vec![entity1], vec![entity2], vec![entity3]]);
            }
            GameObject::Player { direction } => {
                let counter = board.get_eat_counter(position);
                if counter.is_none() {
                    let [entity1, entity2, entity3] = render_object(
                        &mut commands,
                        images.player_images.clone().unwrap(),
                        (
                            current_sprite.0 * 7 + 4,
                            current_sprite.0 * 7 + direction.to_num(),
                            current_sprite.0 * 7 + 5,
                        ),
                        x,
                        y,
                        z,
                        0.,
                        Player,
                    );
                    entity_storage
                        .insert_entities(position, [vec![entity1], vec![entity2], vec![entity3]]);
                } else {
                    let mut counter = counter.unwrap();
                    if counter == 0 {
                        counter = 1;
                    }
                    let [entity1, entity2, entity3, entity4] = render_object_with_sticker(
                        &mut commands,
                        images.player_images.clone().unwrap(),
                        (
                            current_sprite.0 * 7 + 4,
                            current_sprite.0 * 7 + direction.to_num(),
                            current_sprite.0 * 7 + 5,
                        ),
                        25 - counter,
                        x,
                        y,
                        z,
                        0.,
                        Player,
                    );
                    entity_storage.insert_entities(
                        position,
                        [vec![entity1, entity4], vec![entity2], vec![entity3]],
                    );
                }
            }
            GameObject::HidingWall { color, .. } => {
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
                entity_storage
                    .insert_entities(position, [vec![entity1], vec![entity2], vec![entity3]]);
            }
            GameObject::Turtle { color, direction } => {
                let [entity1, entity2, entity3, entity4] = render_object_with_sticker(
                    &mut commands,
                    images.turtle_images.clone().unwrap(),
                    (
                        direction.to_num() * 6 + 1,
                        direction.to_num() * 6,
                        direction.to_num() * 6 + 2,
                    ),
                    4 * 6 + color,
                    x,
                    y,
                    z,
                    0.,
                    Turtle,
                );
                entity_storage.insert_entities(
                    position,
                    [vec![entity1, entity4], vec![entity2], vec![entity3]],
                );
            }
            GameObject::TurtleRock { direction } => {
                let [entity1, entity2, entity3] = render_object_gray(
                    &mut commands,
                    images.turtle_images.clone().unwrap(),
                    (
                        direction.to_num() * 6 + 1,
                        direction.to_num() * 6,
                        direction.to_num() * 6 + 2,
                    ),
                    x,
                    y,
                    z,
                    0.,
                    Turtle,
                );
                entity_storage
                    .insert_entities(position, [vec![entity1], vec![entity2], vec![entity3]]);
            }
            GameObject::TurtleHead {
                direction,
                color: _,
            } => {
                let [entity1, entity2, entity3] = render_object(
                    &mut commands,
                    images.turtle_images.clone().unwrap(),
                    (
                        direction.to_num() * 6 + 4,
                        direction.to_num() * 6 + 3,
                        direction.to_num() * 6 + 5,
                    ),
                    x,
                    y,
                    z,
                    0.,
                    Turtle,
                );
                entity_storage
                    .insert_entities(position, [vec![entity1], vec![entity2], vec![entity3]]);
            }
            GameObject::TeleBox => {
                let [entity1, entity2, entity3] = render_object(
                    &mut commands,
                    images.telebox_images.clone().unwrap(),
                    (1, 0, 2),
                    x,
                    y,
                    z,
                    0.,
                    Box,
                );
                entity_storage
                    .insert_entities(position, [vec![entity1], vec![entity2], vec![entity3]]);
            }
            _ => (),
        }
        render_glue(position, &mut game_data, &images, &mut commands);
    }
    render_floor(commands, &mut game_data, images, current_sprite);
}
