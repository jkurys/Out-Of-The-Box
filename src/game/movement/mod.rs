use crate::{
    consts::MOVE_ANIMATION_TIME,
    state::{DisplayState, MoveState},
};
use bevy::prelude::*;

use ice::handle_ice;
use keyboard::handle_keypress;
use warp::handle_warp;

use crate::game::game_objects::{Box, Player};

use self::{
    animation::GameAnimationPlugin,
    button::handle_button,
    end_move::end_move,
    resources::{AnimationTimer, MoveData},
    turtle::handle_turtle,
};

mod animation;
mod button;
pub mod consts;
mod end_move;
mod events;
mod ice;
mod keyboard;
pub mod resources;
mod try_move;
mod turtle;
mod utils;
mod warp;
mod sort_positions;

use crate::game::movement::try_move::try_move;

use super::game_objects::Turtle;

pub type MovableInQuery = Or<(With<Box>, With<Player>, With<Turtle>)>;
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameAnimationPlugin);
        app.add_systems(
            Update,
            handle_keypress
                .run_if(is_in_game)
                .run_if(in_state(MoveState::Static)),
        );

        app.add_systems(
            Update,
            (try_move, handle_warp)
                .run_if(is_in_game)
                .run_if(in_state(MoveState::Calculating))
                .chain(),
        );

        app.add_systems(
            Update,
            (handle_button, handle_turtle, handle_ice, end_move)
                .run_if(is_in_game)
                .run_if(in_state(MoveState::AfterAnimationCalc))
                .chain(),
        );

        app.insert_resource(AnimationTimer(Timer::from_seconds(
            MOVE_ANIMATION_TIME,
            TimerMode::Once,
        )));
        app.insert_resource(MoveData{ push_atempts: Vec::new(), moves: Vec::new() });
    }
}

pub fn is_in_game(display_state: Res<State<DisplayState>>) -> bool {
    display_state.get() == &DisplayState::Game
}
