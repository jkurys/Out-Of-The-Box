use bevy::prelude::*;

use crate::{board::Board, game::game_objects::GameObject};

use super::{events::TryMoveEvent, utils::can_block_move};

pub fn handle_button(
    mut board: ResMut<Board>,
    mut writer: EventWriter<TryMoveEvent>,
) {
    let buttons = board.get_all_buttons();
    let mut is_clicked = false;
    for (color, button_color) in buttons.into_iter().enumerate() {
        for button_position in button_color {
            let object = board.get_object_type(button_position.position_above());
            if object != GameObject::Empty {
                is_clicked = true;
            }
        }
        let positions_to_move = board.get_hidden_walls_to_move(color, is_clicked);
            
        for (dir, pos) in positions_to_move {
            let mut next_blocks = Vec::new();
            let mut visited_blocks = Vec::new();
            let mut blocks_that_try_move = Vec::new();
            if can_block_move(&board, board.get_block(pos), dir, &mut next_blocks, &mut blocks_that_try_move, &mut visited_blocks) {
                board.modify_toggle(pos);
            }
            writer.send(TryMoveEvent {
                block: board.get_block(pos),
                direction: dir,
                is_weak: false,
                is_long: false,
                position: pos,
            });
        }
        is_clicked = false;
    }
}
