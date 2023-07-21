use bevy::prelude::*;

use crate::{board::Board, menu::level_editor::resources::BoardSize};

pub fn handle_exit(mut board_size: ResMut<BoardSize>, board: Res<Board>) {
    *board_size = board.get_map_size();
}
