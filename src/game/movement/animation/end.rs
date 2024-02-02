use bevy::prelude::*;

use crate::game::movement::resources::AnimationTimer;

pub fn end_animation(mut timer: ResMut<AnimationTimer>) {
    timer.0.reset();
}
