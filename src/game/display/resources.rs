use bevy::prelude::*;

#[derive(Resource)]
pub struct ButtonAnimationTimer(pub Timer);

#[derive(Resource)]
pub struct ButtonState(pub bool);
