use bevy::prelude::*;

use crate::{game::game_objects::GameObject, resources::Board};

pub fn handle_button(
    mut board: ResMut<Board>,
) {
    let buttons = board.get_all_buttons();
    let mut is_clicked = false;
    let mut color = 0;
    for button_color in buttons {
        for button_position in button_color {
            if board.get_object_type(button_position) != GameObject::Empty {
                is_clicked = true;
            }
        }
        if is_clicked {
            board.rise_hiding_wall(color);
        }
        else {
            board.hide_hiding_wall(color);
        }
        color += 1;
        is_clicked = false;
    }
}
