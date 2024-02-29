use bevy::{prelude::*, utils::HashSet};

use crate::{
    board::Board,
    game::game_objects::{Block, GameObject},
};

// use super::events::TryMoveEvent;
use super::resources::*;

pub fn handle_turtle(mut board: ResMut<Board>, mut moves: ResMut<MoveData> /* mut writer: EventWriter<TryMoveEvent> */) {
    let buttons = board.get_all_buttons();
    let mut is_clicked = false;
    let turtles = board.get_all_turtles();
    for (color, button_color) in buttons.into_iter().enumerate() {
        for button_position in button_color {
            if board.get_object_type(button_position) != GameObject::Empty {
                is_clicked = true;
            }
        }
        if is_clicked {
            for (turtle_pos, direction) in turtles[color].iter() {
                let direction = *direction;
                let turtle_head_pos = turtle_pos.next_position(direction);
                match board.get_object_type(turtle_head_pos) {
                    GameObject::TurtleHead {
                        direction: _,
                        color: _,
                    } => (),
                    _ => {
                        // writer.send(TryMoveEvent {
                        //     block: Block {
                        //         positions: HashSet::from([turtle_head_pos]),
                        //     },
                        //     direction,
                        //     is_weak: false,
                        //     insert_after: Some((
                        //         GameObject::TurtleHead { direction, color },
                        //         turtle_head_pos,
                        //     )),
                        // });
                        moves.push_atempts.push(PushAttempt {
                            block: Block {
                                positions: HashSet::from([turtle_head_pos]),
                            },
                            direction,
                            is_weak: false,
                            insert_after: Some((
                                GameObject::TurtleHead { direction, color },
                                turtle_head_pos,
                            )),
                        });
                        board.insert_block(Block {
                            positions: HashSet::from([*turtle_pos, turtle_head_pos]),
                        })
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
                        })
                    }
                }
            }
        } else {
            let turtle_heads = board.get_all_turtle_heads();
            for &(pos, _) in turtle_heads[color].iter() {
                board.delete_object(pos);
            }
        }
        is_clicked = false;
    }
}
