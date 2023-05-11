use bevy::prelude::*;

use crate::resources::{Images, Board};
use crate::utils::offset_coordinate;
use crate::game::game_objects::*;
use crate::consts::*;

use super::render_entity;

pub fn print_board(map: usize, board: &mut ResMut<Board>, parent: &mut ChildBuilder, images: &Res<Images>) {
    let map_size = board.get_map_size(map);
    let bottom_border = offset_coordinate(0, map_size.height as i32);
    let top_border = offset_coordinate(map_size.height as i32 - 1, map_size.height as i32);
    let left_border = offset_coordinate(0, map_size.width as i32);
    let right_border = offset_coordinate(map_size.width as i32 - 1, map_size.width as i32);
    for y in bottom_border..(top_border + 1) {
        for x in left_border..(right_border + 1) {
            let position = Position { x, y };
            let game_object = board.get_object_from_map(position, map);
            match game_object {
                GameObject::Box => {
                    let image = if board.get_floor_from_map(position, map) == Floor::Goal {
                        images.box_on_goal_image.clone()
                    } else {
                        images.box_image.clone()
                    };
                    let entity = render_entity(Box, parent, image, position, OBJECT_Z_INDEX);
                    board.insert_entity(position, entity);
                }
                GameObject::Wall => {
                    render_entity(
                        Wall,
                        parent,
                        images.wall_image.clone(),
                        position,
                        OBJECT_Z_INDEX,
                    );
                }
                GameObject::Player => {
                    let entity = render_entity(
                        Player,
                        parent,
                        images.player_image.clone(),
                        position,
                        OBJECT_Z_INDEX,
                    );
                    board.insert_entity(position, entity);
                }
                GameObject::HidingWall => {
                    let entity = render_entity(
                        HiddenWall,
                        parent,
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
            let position = Position { x, y};
            let floor = board.get_floor_from_map(position, map);
            match floor {
                Floor::Ice => {
                    render_entity(
                        Ice,
                        parent,
                        images.ice_image.clone(),
                        position,
                        FLOOR_Z_INDEX,
                    );
                }
                Floor::Tile => {
                    render_entity(
                        Background,
                        parent,
                        images.tile_image.clone(),
                        position,
                        FLOOR_Z_INDEX,
                    );
                }
                Floor::Goal => {
                    render_entity(
                        Goal,
                        parent,
                        images.goal_image.clone(),
                        position,
                        FLOOR_Z_INDEX,
                    );
                }
                Floor::Warp(_) => {
                    render_entity(
                        Warp,
                        parent,
                        images.warp_image.clone(),
                        position,
                        FLOOR_Z_INDEX,
                    );
                }
                Floor::HiddenWall {
                    hidden_by_default: _,
                } => {
                    render_entity(
                        HiddenWall,
                        parent,
                        images.hidden_wall_image.clone(),
                        position,
                        FLOOR_Z_INDEX,
                    );
                }
                Floor::Button => {
                    render_entity(
                        BoxButton,
                        parent,
                        images.button_image.clone(),
                        position,
                        FLOOR_Z_INDEX,
                    );
                }
            }
        }
    }
}