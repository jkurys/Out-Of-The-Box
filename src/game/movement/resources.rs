use bevy::prelude::*;

#[derive(Resource)]
pub struct AnimationTimer(pub Timer);

#[derive(Resource)]
pub struct CurrentMap(pub usize);
