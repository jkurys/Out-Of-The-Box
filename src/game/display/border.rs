use crate::board::GameData;
use crate::game::display::background::calculate_borders;
use crate::game::game_objects::*;
use bevy::prelude::*;

pub fn insert_border(mut game_data: ResMut<GameData>) {
    let board = &mut game_data.board;
    let (mut bottom_border, mut top_border, mut left_border, mut right_border) =
        calculate_borders(&board);
    // calculate borders gives the last coordinates on board, borders need to be outside
    bottom_border -= 1;
    top_border += 1;
    left_border -= 1;
    right_border += 1;
    // here we adjust left and right borders to avoid overlapping corners
    for x in (left_border + 1)..=(right_border - 1) {
        board.insert_object(
            Position {
                x,
                y: top_border,
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
    for y in bottom_border..=top_border {
        board.insert_object(
            Position {
                x: left_border,
                y,
                z: 1,
            },
            GameObject::Wall,
        );
        board.insert_object(
            Position {
                x: right_border,
                y,
                z: 1,
            },
            GameObject::Wall,
        );
    }
}
