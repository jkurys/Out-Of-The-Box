use super::Board;
use crate::{
    game::game_objects::{Floor, GameObject, Position},
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
