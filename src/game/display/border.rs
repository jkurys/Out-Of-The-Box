use crate::game::game_objects::*;
use crate::game::display::background::calculate_borders;
use crate::board::Board;
use crate::resources::Images;
use super::render_2_5_d::render_object;
use bevy::prelude::*;

pub fn render_border(
    mut commands: Commands,
    mut board: ResMut<Board>,
    images: Res<Images>
) {
    let (bottom_border, top_border, left_border, right_border) = calculate_borders(&board);
    let map = board.get_current_map();
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
