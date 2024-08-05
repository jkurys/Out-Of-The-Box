use bevy::prelude::*;

use crate::{state::MoveState, game::display::{despawn_board, background::render_board, border::render_border}};

use self::{cont::continue_animation, end::end_animation, frame::move_animation};

use super::is_in_game;

mod cont;
mod end;
mod frame;

pub struct GameAnimationPlugin;

impl Plugin for GameAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MoveState::Animation), 
        (
            despawn_board,
            render_board,
            render_border,
        )
        );

        app.add_systems(
            Update,
            (move_animation, continue_animation)
                .run_if(is_in_game)
                .run_if(in_state(MoveState::Animation))
                .chain(),
        );

        app.add_systems(
            OnExit(MoveState::Animation),
            end_animation.run_if(is_in_game),
        );
    }
}
