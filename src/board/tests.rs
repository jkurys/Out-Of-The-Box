use bevy::prelude::*;

use super::Board;
use crate::{
    game::{game_objects::{Floor, GameObject, Position, Player}, movement::MovementPlugin},
    menu::level_editor::resources::BoardSize,
};

#[test]
pub fn insert_does_nothing_outside_borders() {
    let mut board = Board::new();
    board.set_map_size(BoardSize {
        width: 11,
        height: 11,
    });
    let pos1 = Position { x: 11, y: 11 };
    board.insert_floor(pos1, Floor::Ice);
    assert_eq!(board.get_floor_type(pos1), Floor::Tile);
    board.insert_object(pos1, GameObject::Box);
    assert_eq!(board.get_object_type(pos1), GameObject::Empty);
}

#[test]
pub fn basic_test() {
    assert_eq!(15, 10 + 5);
}

#[test]
pub fn basic_move_test() {
    let mut board = Board::new();
    board.set_map_size(BoardSize {
        width: 11,
        height: 11,
    });
    board.insert_object(Position {x: 6, y: 6}, GameObject::Player);

    let mut app = App::new();
    app.add_plugins(MovementPlugin);
    let mut input = ButtonInput::<KeyCode>::default();
    input.press(KeyCode::KeyA);
    app.insert_resource(board);
    app.insert_resource(input);
    app.update();
    let mut world = app.world;
    let players = world.query::<&Player>().single(&world);
    assert_eq!(players, &Player);
}
