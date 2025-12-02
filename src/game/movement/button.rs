use bevy::prelude::*;

use crate::{board::GameData, game::game_objects::GameObject};

use super::{events::TryMoveEvent, strong::can_block_move};

pub fn handle_button(mut writer: EventWriter<TryMoveEvent>, mut game_data: ResMut<GameData>) {
    let buttons = game_data.board.get_all_buttons();
    let mut is_clicked = false;
    for (color, button_color) in buttons.into_iter().enumerate() {
        for button_position in button_color {
            let object = game_data
                .board
                .get_object_type(button_position.position_above());
            if object != GameObject::Empty {
                is_clicked = true;
            }
        }
        let positions_to_move = game_data.board.get_hidden_walls_to_move(color, is_clicked);

        for (dir, pos) in positions_to_move {
            let block = game_data.board.get_block(pos);
            if can_block_move(&mut game_data, block, dir) {
                game_data.modify_toggle(pos);
            }
            writer.send(TryMoveEvent {
                block,
                direction: dir,
                is_weak: false,
                is_long: false,
                position: pos,
            });
        }
        is_clicked = false;
    }
}
