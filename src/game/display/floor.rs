use bevy::prelude::*;

use crate::consts::*;
use crate::game::game_objects::{Background, Button, *};
use crate::resources::{Images, CurrentSprite};

use crate::board::Board;

use crate::game::display::background::calculate_borders;
use super::render_2_5_d::{render_object, render_object_with_sticker, render_sticker};


pub fn render_floor(
    mut commands: Commands,
    board: ResMut<Board>,
    images: Res<Images>,
    current_sprite: Res<CurrentSprite>,
) {
    let (bottom_border, top_border, left_border, right_border) = calculate_borders(&board);
    for y in bottom_border..=top_border {
        for x in left_border..=right_border {
            let z = 0;
            let position = Position { x, y, z };
            let floor = board.get_floor_type(position);
            match floor {
                Floor::Ice => {
                    // render_entity(
                    //     Ice,
                    //     &mut commands,
                    //     images.ice_image.clone(),
                    //     position,
                    //     FLOOR_Z_INDEX,
                    // );
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
                    // render_entity(
                    //     Background,
                    //     &mut commands,
                    //     images.tile_image.clone(),
                    //     position,
                    //     FLOOR_Z_INDEX,
                    // );
                    // render_object(
                    //     &mut commands,
                    //     images.wall_images.clone().unwrap(),
                    //     (1, 0 ,2),
                    //     x,
                    //     y,
                    //     z,
                    //     0.,
                    //     Background,
                    // );
                }
                Floor::Goal => {
                    render_object_with_sticker(
                        &mut commands,
                        images.wall_images.clone().unwrap(),
                        (1, 0, 2),
                        3,
                        x,
                        y,
                        z,
                        0.,
                        Goal,
                    );
                }
                Floor::Dirt => {
                    render_object_with_sticker(
                        &mut commands,
                        images.wall_images.clone().unwrap(),
                        (1, 0, 2),
                        7,
                        x,
                        y,
                        z,
                        0.,
                        Dirt,
                    );
                }
                Floor::Void => {
                    render_object(
                        &mut commands,
                        images.water_images.clone().unwrap(),
                        (1, 0 ,2),
                        x,
                        y,
                        z,
                        -0.5,
                        Water,
                    );
                    // render_entity(
                    //     Water,
                    //     &mut commands,
                    //     images.water_image.clone(),
                    //     position,
                    //     FLOOR_Z_INDEX,
                    // );
                }
                Floor::Warp(_) => {
                    // render_entity(
                    //     Warp,
                    //     &mut commands,
                    //     images.warp_image.clone(),
                    //     position,
                    //     FLOOR_Z_INDEX,
                    // );
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
  // 
  //                   spawn_from_atlas(
  //                       &mut commands,
  //                       images.hidden_wall_images.clone().unwrap(),
  //                       color * 4 + 3,
  //                       position.x,
  //                       position.y,
  //                       FLOOR_Z_INDEX,
  //                       HiddenWall,
  //                   );
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
                    // if color == 0 {
                    //     render_entity(
                    //         Button,
                    //         &mut commands,
                    //         images.tile_image.clone(),
                    //         position,
                    //         FLOOR_Z_INDEX,
                    //     );
                    //     let img = asset_server.load("textures/button_red_big.png");
                    //     render_entity(Button, &mut commands, img, position, FLOOR_STICKER_Z_INDEX);
                    // }
                    // else {
                    // render_entity(
                    //     Button,
                    //     &mut commands,
                    //     images.tile_image.clone(),
                    //     position,
                    //     FLOOR_Z_INDEX,
                    // );
                    // spawn_from_atlas(
                    //     &mut commands,
                    //     images.button_images.clone().unwrap(),
                    //     color,
                    //     position.x,
                    //     position.y,
                    //     FLOOR_STICKER_Z_INDEX,
                    //     Button,
                    // );
                    // }
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
                    // spawn_from_atlas(
                    //     &mut commands,
                    //     atlas,
                    //     index,
                    //     position.x,
                    //     position.y,
                    //     FLOOR_Z_INDEX,
                    //     Background,
                    //     // NOTE: if it makes any difference later change this
                    // );
                }
            }
        }
    }
}


