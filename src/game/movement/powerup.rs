use bevy::prelude::*;

use crate::{
    board::Board,
    game::game_objects::{Block, Direction, GameObject, Position, PowerUpType},
};

use super::{events::EnteredFloorEvent, resources::DisplayButton, utils::perform_move};


pub fn eat_powerup(
    powerup: PowerUpType,
    position: Position,
    block: Block,
    board: &mut ResMut<Board>,
    direction: Direction,
    writer: &mut EventWriter<EnteredFloorEvent>,
    is_weak: bool,
    display_button: &mut ResMut<DisplayButton>,
) {
    let eaten = board.get_all_eat();
    let opt = eaten.get(&position);
    if opt.is_none() {
        board.delete_object(position);
        board.insert_object(position, GameObject::Player { powerup: Some(powerup), direction });
    } else {
        let player = board.get_object_type(position);
        if let GameObject::Player { powerup: _, direction: new_direction } = player {
            board.delete_object(position);
            board.insert_object(position, GameObject::Player { powerup: Some(powerup), direction: new_direction });
        }
    }
    display_button.0 = true;
    perform_move(vec![block], board, direction, writer, is_weak);
}

