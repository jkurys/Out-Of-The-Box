use bevy::prelude::*;

use crate::{board::Board, game::game_objects::GameObject};

pub fn handle_button(mut board: ResMut<Board>) {
    let buttons = board.get_all_buttons();
    let mut is_clicked = false;
    for (color, button_color) in buttons.into_iter().enumerate() {
        for button_position in button_color {
            let object = board.get_object_type(button_position);
            if object != GameObject::Empty {
                is_clicked = true;
            }
        }
        if is_clicked {
            board.rise_hiding_wall(color);
        } else {
            board.hide_hiding_wall(color);
        }
        is_clicked = false;
    }
}
