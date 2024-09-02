use bevy::{prelude::*, utils::HashSet};

use crate::{
    board::Board,
    game::game_objects::{Block, GameObject},
};

use super::{events::TryMoveEvent, utils::can_block_move};

// BUG: double turtle in lower right on level 8 moves too far
pub fn handle_turtle(
    mut board: ResMut<Board>,
    mut writer: EventWriter<TryMoveEvent>,
    mut button_state: Local<[bool; 3]>,
) {
    let buttons = board.get_all_buttons();
    let turtles = board.get_all_turtles();
    let mut is_clicked = false;
    for (color, button_color) in buttons.clone().into_iter().enumerate() {
        if button_state[color] {
            for (turtle_pos, dir) in turtles[color].iter() {
                if buttons[color].contains(&turtle_pos.next_position(*dir).position_below()) {
                    is_clicked = true;
                }
            }
        }
        for button_position in button_color {
            if board.get_object_type(button_position.position_above()) != GameObject::Empty {
                is_clicked = true;
            }
        }
        if is_clicked {
            // BUG: jak jest kratka odstepu miedzy rzuwiami co sie na siebie patrza to sie pojawia
            // jedna z glow
            for (turtle_pos, direction) in turtles[color].iter() {
                let direction = *direction;
                let turtle_head_pos = turtle_pos.next_position(direction);
                match board.get_object_type(turtle_head_pos) {
                    GameObject::TurtleHead {
                        direction: _,
                        color: _,
                    } => (),
                    GameObject::Empty => {
                        board.insert_object(
                            turtle_head_pos,
                            GameObject::TurtleHead { direction, color },
                        );
                        board.insert_block(Block {
                            positions: HashSet::from([turtle_head_pos, *turtle_pos]),
                        });
                    }
                    _ => {
                        let mut empty_vec = Vec::new();
                        let mut empty_vec2 = Vec::new();
                        if can_block_move(&board, board.get_block(turtle_head_pos), direction, &mut empty_vec, &mut empty_vec2) {
                            writer.send(TryMoveEvent {
                                block: board.get_block(turtle_head_pos),
                                position: turtle_head_pos,
                                direction,
                                is_weak: false,
                            });
                        }
                        else {
                            writer.send(TryMoveEvent {
                                block: board.get_block(*turtle_pos),
                                position: *turtle_pos,
                                direction: direction.opposite(),
                                is_weak: false,
                            });
                        }
                        board.delete_block(&Block {
                            positions: HashSet::from([*turtle_pos]),
                        });
                    }
                }
            }
            let turtle_heads = board.get_all_turtle_heads();
            for &(pos, dir) in turtle_heads[color].iter() {
                match board.get_object_type(pos.prev_position(dir)) {
                    GameObject::Turtle {
                        direction: inside_dir,
                        color: _,
                    } if inside_dir == dir => (),
                    _ => {
                        board.delete_object(pos);
                        board.delete_block(&Block {
                            positions: HashSet::from([pos, pos.prev_position(dir)]),
                        });
                    }
                }
            }
            button_state[color] = true;
        } else if !is_clicked && button_state[color] {
            let turtle_heads = board.get_all_turtle_heads();
            for &(pos, dir) in turtle_heads[color].iter() {
                board.delete_object(pos);
                board.delete_block(&Block {
                    positions: HashSet::from([pos, pos.prev_position(dir)]),
                });
            }
            button_state[color] = false;
        }
        is_clicked = false;
    }
}
