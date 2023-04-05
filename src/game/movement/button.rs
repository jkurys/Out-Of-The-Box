use bevy::prelude::*;

use crate::{game::game_objects::Floor, resources::Board};

use super::events::{EnteredFloorEvent, ExitedFloorEvent};

pub fn handle_button(
    mut entered_reader: EventReader<EnteredFloorEvent>,
    mut exited_reader: EventReader<ExitedFloorEvent>,
    mut board: ResMut<Board>,
) {
    let mut do_once = false;
    for event in exited_reader.iter() {
        if event.floor == Floor::Button && !do_once {
            board.hide_hiding_wall();
            do_once = true;
        }
    }
    do_once = false;
    for event in entered_reader.iter() {
        if event.floor == Floor::Button && !do_once {
            board.rise_hiding_wall();
            do_once = true;
        }
    }
}
