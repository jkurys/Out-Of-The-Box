use bevy::prelude::*;

#[derive(Resource)]
pub struct AnimationTimer(pub Timer);

#[derive(Resource)]
pub struct CurrentMap(pub usize);

#[derive(Resource)]
pub struct FireAnimation(pub bool);
