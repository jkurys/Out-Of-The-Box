use bevy::prelude::*;

use crate::consts::*;
use crate::game::game_objects::{Background, Button, *};
use crate::resources::{Images, CurrentSprite};

use crate::board::Board;

use super::render_2_5_d::{render_object, render_sticker};


pub fn render_floor(
    mut commands: Commands,
    mut board: ResMut<Board>,
    images: Res<Images>,
    current_sprite: Res<CurrentSprite>,
) {
    let floors = board.get_floors();
    for (&position, &floor) in floors.iter() {
        let Position { x, y, z } = position;
        match floor {
            Floor::Ice => {
                render_object(
                    &mut commands,
                    images.ice_images.clone().unwrap(),
                    (1, 0 ,2),
                    x,
                    y,
                    z,
                    0.01,
                    Ice,
                );
            }
            Floor::Tile => {
            }
            Floor::Goal => {
                let entity = render_sticker(
                    &mut commands,
                    3,
                    x,
                    y,
                    z,
                    images.wall_images.clone().unwrap(),
                    Goal,
                    UPPER_HALF_STICKER_Z_INDEX,
                );
                board.append_entities(position, [vec![entity], vec![], vec![]]);
            }
            Floor::Dirt => {
                let entity = render_sticker(
                    &mut commands,
                    7,
                    x,
                    y,
                    z,
                    images.wall_images.clone().unwrap(),
                    Dirt,
                    UPPER_HALF_STICKER_Z_INDEX,
                );
                board.append_entities(position, [vec![entity], vec![], vec![]]);
            }
            Floor::Void => {
                render_object(
                    &mut commands,
                    images.water_images.clone().unwrap(),
                    (1, 0 ,2),
                    x,
                    y,
                    z,
                    -0.1,
                    Water,
                );
            }
            Floor::Warp(_) => {
            }
            Floor::HiddenWall {
                hidden_by_default: _,
                color,
            } => {
                render_object(
                    &mut commands,
                    images.hidden_wall_images.clone().unwrap(),
                    (color * 3 + 1, color * 3, color * 3 + 2),
                    x,
                    y,
                    z,
                    0.01,
                    HiddenWall,
                );
            }
            Floor::Button(color) => {
                render_object(
                    &mut commands,
                    images.wall_images.clone().unwrap(),
                    (1, 0 ,2),
                    x,
                    y,
                    z,
                    0.,
                    Button,
                );
                render_sticker(
                    &mut commands,
                    4 + color,
                    x,
                    y,
                    z,
                    images.wall_images.clone().unwrap(),
                    Button,
                    UPPER_HALF_STICKER_Z_INDEX,
                );
            }
            Floor::Obj(obj) => {
                let indices = get_obj_indices(obj, &current_sprite);
                let atlas_opt = get_obj_img(obj, &images);
                let atlas = atlas_opt.unwrap();
                render_object(
                    &mut commands,
                    atlas,
                    indices,
                    x,
                    y,
                    z,
                    0.,
                    Background
                );
            }
        }
    }
}


