use bevy::prelude::*;

use crate::{game::game_objects::GameObject, resources::Board};

pub fn handle_button(
    mut board: ResMut<Board>,
) {
    let buttons = board.get_all_buttons();
    let mut is_clicked = false;
    for button_position in buttons {
        if board.get_object_type(button_position) != GameObject::Empty {
            is_clicked = true;
        }
    }
    if is_clicked {
        board.rise_hiding_wall();
    }
    else {
        board.hide_hiding_wall();
    }
}
