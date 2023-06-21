use bevy::prelude::*;

use crate::state::MoveState;

use self::{animation::move_animation, cont::continue_animation, end::end_animation};

use super::is_in_game;

mod animation;
mod cont;
mod end;

pub struct GameAnimationPlugin;

impl Plugin for GameAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (move_animation, continue_animation)
                .distributive_run_if(is_in_game)
                .chain()
                .in_set(OnUpdate(MoveState::Animation)),
        );

        app.add_system(
            end_animation
                .run_if(is_in_game)
                .in_schedule(OnExit(MoveState::Animation)),
        );
    }
}
