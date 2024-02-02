use bevy::prelude::*;

use crate::{game::movement::resources::AnimationTimer, state::MoveState};

pub fn continue_animation(
    mut app_state: ResMut<NextState<MoveState>>,
    mut timer: ResMut<AnimationTimer>,
) {
    if !timer.0.finished() {
        return;
    }
    timer.0.reset();
    app_state.set(MoveState::AfterAnimationCalc);
}
