use bevy::prelude::*;

use crate::{game::game_objects::{GameObject, Direction}, board::Board};

pub fn handle_turtle(mut board: ResMut<Board>) {
    let buttons = board.get_all_buttons();
    let mut is_clicked = false;
    let mut color = 0;
    let turtles = board.get_all_turtles();
    for button_color in buttons {
        for button_position in button_color {
            if board.get_object_type(button_position) != GameObject::Empty {
                is_clicked = true;
            }
        }
        if is_clicked {
            for turtle_pos in turtles[color].iter() {
                
                board.insert_object(turtle_pos.next_position(Direction::Left), GameObject::TurtleHead);
            }
        } else {
            for &turtle_pos in turtles[color].iter() {
                let turtle_head_pos = turtle_pos.next_position(Direction::Left);
                if let GameObject::TurtleHead = board.get_object_type(turtle_head_pos) {
                    board.delete_object(turtle_head_pos);
                }
            }
        }
        color += 1;
        is_clicked = false;
    }
}