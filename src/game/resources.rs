use crate::board::Board;
use bevy::prelude::*;

#[derive(Resource)]
pub struct BoardStates {
    pub boards: Vec<Board>,
}

#[derive(Resource)]
pub struct VictoryTimer(pub Timer);
