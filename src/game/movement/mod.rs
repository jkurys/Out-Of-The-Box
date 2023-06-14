use crate::{
    consts::MOVE_ANIMATION_TIME,
    state::{MoveState, DisplayState},
};
use bevy::prelude::*;

use animation::{end_animation, move_animation};
use ice::handle_ice;
use keyboard::handle_keypress;
use warp::handle_warp;

use crate::game::game_objects::{Box, Player};

use self::{
    button::handle_button,
    events::{EnteredFloorEvent, ExitedFloorEvent},
    position_updating::handle_move,
    resources::AnimationTimer,
};

mod animation;
mod button;
pub mod consts;
mod events;
mod ice;
mod keyboard;
mod position_updating;
pub mod resources;
mod warp;

pub type MovableInQuery = Or<(With<Box>, With<Player>)>;
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            handle_move,
            move_animation,
            handle_button,
            handle_ice,
            handle_warp,
            continue_animation,
        ).distributive_run_if(is_in_game)
            .chain()
            .in_set(OnUpdate(MoveState::Moving)));

        app.add_system(end_animation
            .run_if(is_in_game)
            .in_schedule(OnExit(MoveState::Moving))
        );

        app.add_system(handle_keypress
            .run_if(is_in_game)
            .in_set(OnUpdate(MoveState::Static))
        );

        app.add_event::<ExitedFloorEvent>();
        app.init_resource::<Events<EnteredFloorEvent>>();
        app.insert_resource(AnimationTimer(Timer::from_seconds(
            MOVE_ANIMATION_TIME,
            TimerMode::Once,
        )));
    }
}

pub fn is_in_game(
    display_state: Res<State<DisplayState>>,
) -> bool {
    display_state.0 == DisplayState::Game
}

fn continue_animation(
    mut app_state: ResMut<NextState<MoveState>>,
    mut timer: ResMut<AnimationTimer>,
    reader: EventReader<ExitedFloorEvent>,
    mut entered_events: ResMut<Events<EnteredFloorEvent>>,
) {
    if !timer.0.finished() {
        return;
    }

    entered_events.update();
    if !reader.is_empty() {
        timer.0.reset();
    } else {
        app_state.set(MoveState::Static);
    }
}
