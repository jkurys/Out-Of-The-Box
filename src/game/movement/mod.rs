use crate::{
    consts::MOVE_ANIMATION_TIME,
    state::{DisplayState, MoveState}, board::Board,
};
use bevy::prelude::*;

use ice::handle_ice;
use keyboard::handle_keypress;

use self::{
    animation::GameAnimationPlugin,
    button::handle_button,
    end_move::end_move,
    events::{EnteredFloorEvent, TryMoveEvent},
    resources::{AnimationTimer, FireAnimation, DisplayButton},
    turtle::handle_turtle,
    spit::handle_spit, fall::handle_fall,
};

mod animation;
mod button;
pub mod consts;
mod end_move;
mod events;
mod ice;
mod keyboard;
pub mod resources;
mod sort_positions;
mod try_move;
mod turtle;
mod utils;
mod spit;
mod fall;

use crate::game::movement::try_move::try_move;

use super::{
    display::{
        background::render_board,
        border::render_border,
        despawn_board,
    },
    game_objects::{Box, Glue, Player, Turtle, HiddenWall, Goal, PowerUp, Dirt},
};

pub type MovableInQuery = Or<(With<Box>, With<Player>, With<Turtle>, With<Glue>, With<HiddenWall>, With<Goal>, With<Dirt>, With<PowerUp>)>;
pub struct MovementPlugin;

#[derive(Resource, Clone)]
pub struct BoardPreMove(Board);

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameAnimationPlugin);
        app.add_systems(
            Update,
            (
                handle_keypress,
            )
                .run_if(is_in_game)
                .run_if(in_state(MoveState::Static)),
        );

        app.add_systems(
            Update,
            (
                despawn_board,
                render_board,
                render_border,
                handle_spit,
                try_move,
            )
                .run_if(is_in_game)
                .run_if(in_state(MoveState::Calculating))
                .chain(),
        );

        app.add_systems(
            Update,
            (
                handle_fall,
                handle_turtle,
                handle_button,
                handle_ice,
                end_move
            )
                .run_if(is_in_game)
                .run_if(in_state(MoveState::AfterAnimationCalc))
                .chain(),
        );

        app.add_event::<TryMoveEvent>();
        app.init_resource::<Events<EnteredFloorEvent>>();
        app.insert_resource(FireAnimation(false));
        app.insert_resource(BoardPreMove(Board::new()));
        app.insert_resource(AnimationTimer(Timer::from_seconds(
            MOVE_ANIMATION_TIME,
            TimerMode::Once,
        )));
        app.insert_resource(DisplayButton(false));
    }
}

pub fn is_in_game(display_state: Res<State<DisplayState>>) -> bool {
    display_state.get() == &DisplayState::Game
}
