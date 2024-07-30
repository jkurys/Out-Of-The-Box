use bevy::prelude::*;

use crate::consts::*;
use crate::game::game_objects::{Button, *};
use crate::resources::Images;

use crate::board::Board;

use crate::game::display::background::calculate_borders;
use super::{render_entity, spawn_from_atlas};


pub fn render_floor(
    mut commands: Commands,
    board: ResMut<Board>,
    images: Res<Images>,
) {
    let (bottom_border, top_border, left_border, right_border) = calculate_borders(&board);
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
                Floor::Dirt => {
                    render_entity(
                        Dirt,
                        &mut commands,
                        images.tile_image.clone(),
                        position,
                        FLOOR_Z_INDEX,
                    );
                    render_entity(
                        Dirt,
                        &mut commands,
                        images.dirt_image.clone(),
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


