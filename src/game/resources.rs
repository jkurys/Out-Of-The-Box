use crate::board::Board;
use bevy::prelude::*;

use super::game_objects::Direction;

#[derive(Resource)]
pub struct BoardStates {
    pub boards: Vec<Board>,
}

#[derive(Resource)]
pub struct PlayerDirection(pub Direction);

#[derive(Resource)]
pub struct VictoryTimer(pub Timer);
