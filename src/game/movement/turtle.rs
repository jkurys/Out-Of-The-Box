use bevy::prelude::*;

use crate::{
    board::GameData, consts::NUMBER_OF_COLORS, game::game_objects::Direction, game::game_objects::*,
};

use super::{events::TryMoveEvent, strong::can_block_move};

pub fn handle_turtle(
    mut game_data: ResMut<GameData>,
    mut writer: EventWriter<TryMoveEvent>,
    mut button_state: Local<[bool; NUMBER_OF_COLORS]>,
) {
    let objects = game_data.board.get_objects();
    let floors = game_data.board.get_floors();
    let mut buttons: [Vec<Position>; NUMBER_OF_COLORS] = [const { Vec::new() }; NUMBER_OF_COLORS];
    for (position, floor) in floors.iter() {
        if let Floor::Button(color) = floor {
            buttons[*color].push(*position);
        }
    }
    let turtles: [Vec<(Position, Direction)>; NUMBER_OF_COLORS] = {
        let mut temp: [Vec<(Position, Direction)>; NUMBER_OF_COLORS] =
            [const { Vec::new() }; NUMBER_OF_COLORS];
        for (position, object) in objects.iter() {
            if let GameObject::Turtle { direction, color } = object {
                temp[*color].push((*position, *direction));
            }
        }
        temp
    };
    let turtle_heads = {
        let mut temp: [Vec<(Position, Direction)>; NUMBER_OF_COLORS] =
            [const { Vec::new() }; NUMBER_OF_COLORS];
        let objects = game_data.board.get_objects();
        for (position, object) in objects.iter() {
            if let GameObject::TurtleHead { direction, color } = object {
                temp[*color].push((*position, *direction));
            }
        }
        temp
    };
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
            if game_data
                .board
                .get_object_type(button_position.position_above())
                != GameObject::Empty
            {
                is_clicked = true;
            }
        }
        if is_clicked {
            // BUG: jak jest kratka odstepu miedzy rzuwiami co sie na siebie patrza to sie pojawia
            // jedna z glow
            for (turtle_pos, direction) in turtles[color].iter() {
                let direction = *direction;
                let turtle_head_pos = turtle_pos.next_position(direction);
                let turtle_head_block = game_data.board.get_block(turtle_head_pos);
                let turtle_block = game_data.board.get_block(*turtle_pos);
                match game_data.board.get_object_type(turtle_head_pos) {
                    GameObject::TurtleHead {
                        direction: _,
                        color: _,
                    } => (),
                    GameObject::Empty => {
                        game_data.board.insert_object(
                            turtle_head_pos,
                            GameObject::TurtleHead { direction, color },
                        );
                        game_data.board.insert_block(Block {
                            positions: SmallSet::from([turtle_head_pos, *turtle_pos]),
                        });
                    }
                    _ => {
                        if can_block_move(&mut game_data, turtle_head_block, direction) {
                            writer.send(TryMoveEvent {
                                block: turtle_head_block,
                                position: turtle_head_pos,
                                direction,
                                is_weak: false,
                                is_long: false,
                            });
                        } else if can_block_move(&mut game_data, turtle_block, direction.opposite())
                        {
                            writer.send(TryMoveEvent {
                                block: game_data.board.get_block(*turtle_pos),
                                position: *turtle_pos,
                                direction: direction.opposite(),
                                is_weak: false,
                                is_long: false,
                            });
                        } else {
                            let GameData {
                                board,
                                entity_storage,
                            } = &mut *game_data;
                            board.delete_object(*turtle_pos, entity_storage);
                            board.insert_object(*turtle_pos, GameObject::TurtleRock { direction })
                            // insert turtle rock
                        }
                        game_data.board.delete_block(&Block {
                            positions: SmallSet::from([*turtle_pos]),
                        });
                    }
                }
            }
            for &(pos, dir) in turtle_heads[color].iter() {
                match game_data.board.get_object_type(pos.prev_position(dir)) {
                    GameObject::Turtle {
                        direction: inside_dir,
                        color: _,
                    } if inside_dir == dir => (),
                    _ => {
                        let GameData {
                            board,
                            entity_storage,
                        } = &mut *game_data;

                        board.delete_object(pos, &mut *entity_storage);
                        board.delete_block(&Block {
                            positions: SmallSet::from([pos, pos.prev_position(dir)]),
                        });
                    }
                }
            }
            button_state[color] = true;
        } else if !is_clicked && button_state[color] {
            for &(pos, dir) in turtle_heads[color].iter() {
                let GameData {
                    board,
                    entity_storage,
                } = &mut *game_data;
                board.delete_object(pos, &mut *entity_storage);
                board.delete_block(&Block {
                    positions: SmallSet::from([pos, pos.prev_position(dir)]),
                });
            }
            button_state[color] = false;
        }
        is_clicked = false;
    }
}
