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
    events::{EnteredFloorEvent, TryMoveEvent},
    resources::AnimationTimer,
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

use crate::game::movement::try_move::try_move;

use super::game_objects::Turtle;

pub type MovableInQuery = Or<(With<Box>, With<Player>, With<Turtle>)>;
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(GameAnimationPlugin);
        app.add_system(
            handle_keypress
                .run_if(is_in_game)
                .in_set(OnUpdate(MoveState::Static)),
        );

        app.add_systems(
            (try_move, handle_warp)
                .distributive_run_if(is_in_game)
                .chain()
                .in_set(OnUpdate(MoveState::Calculating)),
        );

        app.add_systems(
            (handle_button, handle_turtle, handle_ice, end_move)
                .distributive_run_if(is_in_game)
                .chain()
                .in_set(OnUpdate(MoveState::AfterAnimationCalc)),
        );

        app.add_event::<TryMoveEvent>();
        app.init_resource::<Events<EnteredFloorEvent>>();
        app.insert_resource(AnimationTimer(Timer::from_seconds(
            MOVE_ANIMATION_TIME,
            TimerMode::Once,
        )));
    }
}

pub fn is_in_game(display_state: Res<State<DisplayState>>) -> bool {
    display_state.0 == DisplayState::Game
}
