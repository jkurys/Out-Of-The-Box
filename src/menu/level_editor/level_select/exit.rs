use bevy::prelude::*;

use crate::{board::GameData, menu::level_editor::resources::BoardSize};

pub fn handle_exit(mut board_size: ResMut<BoardSize>, game_data: Res<GameData>) {
    *board_size = game_data.board.get_map_size();
}
