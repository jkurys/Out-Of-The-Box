use bevy::prelude::*;

use crate::{
    board::Board,
    game::game_objects::GameObject,
};

use super::events::TryMoveEvent;

pub fn handle_turtle(mut board: ResMut<Board>, mut writer: EventWriter<TryMoveEvent>) {
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
            for (turtle_pos, direction) in turtles[color].iter() {
                let direction = *direction;
                let turtle_head_pos = turtle_pos.next_position(direction);
                if board.get_object_type(turtle_head_pos) != (GameObject::TurtleHead { direction }) {
                    writer.send(TryMoveEvent {
                        position: turtle_head_pos,
                        direction,
                        is_weak: false,
                        insert_after: Some((GameObject::TurtleHead { direction }, turtle_head_pos)),
                    });
                }
            }
        } else {
            for (turtle_pos, direction) in turtles[color].iter() {
                let direction = *direction;
                let turtle_head_pos = turtle_pos.next_position(direction);
                if board.get_object_type(turtle_head_pos) == (GameObject::TurtleHead { direction }) {
                    board.delete_object(turtle_head_pos);
                }
            }
        }
        color += 1;
        is_clicked = false;
    }
}
