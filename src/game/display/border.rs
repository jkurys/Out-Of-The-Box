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
    let (mut bottom_border, mut top_border, mut left_border, mut right_border) = calculate_borders(&board);
    bottom_border -= 1;
    top_border += 1;
    left_border -= 1;
    right_border += 1;
    //spawn horizontal border for the level and insert it to board
    for x in left_border..=right_border {
        render_object(
            &mut commands,
            images.wall_images.clone().unwrap(),
            (1, 0, 2),
            x,
            top_border,
            1,
            0.,
            Wall,
        );
        board.insert_object_unchecked(Position { x, y: top_border, z: 1 }, GameObject::Wall);
    }
    for y in (bottom_border..=top_border).rev() {
        render_object(
            &mut commands,
            images.wall_images.clone().unwrap(),
            (1, 0, 2),
            left_border,
            y,
            1,
            0.,
            Wall,
        );
        render_object(
            &mut commands,
            images.wall_images.clone().unwrap(),
            (1, 0, 2),
            right_border,
            y,
            1,
            0.,
            Wall,
        );
        board.insert_object_unchecked(Position { x: left_border, y, z: 1 }, GameObject::Wall);
        board.insert_object_unchecked(
            Position { x: right_border, y, z: 1 },
            GameObject::Wall,
        );
    }
    //spawn vertical borders for the level and insert it to board
    for x in left_border..=right_border {
        render_object(
            &mut commands,
            images.wall_images.clone().unwrap(),
            (1, 0, 2),
            x,
            bottom_border,
            1,
            0.,
            Wall,
        );
        board.insert_object_unchecked(
            Position {
                x,
                y: bottom_border,
                z: 1,
            },
            GameObject::Wall,
        );
        board.insert_object(
            Position {
                x,
                y: bottom_border,
                z: 1,
            },
            GameObject::Wall,
        );
    }
}
