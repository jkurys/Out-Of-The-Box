use bevy::prelude::*; 
use crate::board::Board;

#[derive(Resource)]
pub struct BoardStates {
    pub boards: Vec<Board>,
}

#[derive(Resource)]
pub struct VictoryTimer(pub Timer);

