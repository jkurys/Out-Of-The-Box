use bevy::prelude::*;
use crate::game::game_objects::{
    Block, 
    Direction, 
    Floor, 
    GameObject, 
    Position
};

#[derive(Resource)]
pub struct AnimationTimer(pub Timer);

#[derive(Resource)]
pub struct CurrentMap(pub usize);

#[derive(Resource, Clone)]
pub struct MoveData {
    pub push_atempts: Vec<PushAttempt>,
    pub moves: Vec<MoveObject>,
}

#[derive(Clone)]
pub struct PushAttempt {
    pub block: Block,
    pub direction: Direction,
    pub is_weak: bool,
    pub insert_after: Option<(GameObject, Position)>,
}

#[derive(Clone)]
pub struct MoveObject {
    pub floor: Floor,
    pub position: Position,
    pub object: GameObject,
    pub direction: Direction,
}
