use bevy::prelude::*;

use crate::game::game_objects::Position;

#[derive(Resource)]
pub struct AnimationTimer(pub Timer);

#[derive(Resource)]
pub struct FireAnimation(pub bool);

#[derive(Resource)]
pub struct DisplayButton(pub bool);

#[derive(Resource)]
pub struct TeleportPositions(pub Option<[Position; 2]>);

#[derive(Resource)]
pub struct TeleportFirst(pub bool);
