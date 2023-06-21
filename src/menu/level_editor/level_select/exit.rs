use bevy::prelude::*;

use crate::{menu::level_editor::resources::BoardSize, board::Board};

pub fn handle_exit(mut board_size: ResMut<BoardSize>, board: Res<Board>) {
    *board_size = board.get_map_size();
}