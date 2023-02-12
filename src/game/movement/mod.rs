use crate::{
    consts::MOVE_ANIMATION_TIME,
    labels::Labels,
    state::{GameState, Move},
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

use super::display::{
    background::{render_board, render_border},
    despawn_board,
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
        app.add_system_set(
            SystemSet::on_update(GameState(Some(Move::Moving)))
                .with_system(handle_move.before(move_animation))
                .with_system(
                    move_animation
                        .before(handle_warp)
                        .before(handle_ice)
                        .before(handle_button),
                )
                .with_system(handle_button.before(despawn_board))
                .with_system(handle_warp.before(despawn_board))
                .with_system(handle_ice.before(despawn_board)) //otherwise it could ignore the positions_on_ice and end the animation
                .with_system(despawn_board.before(render_board).before(render_border))
                .with_system(render_board.before(continue_animation))
                .with_system(render_border.before(continue_animation))
                .with_system(continue_animation),
        )
        .add_system_set(
            SystemSet::on_exit(GameState(Some(Move::Moving))).with_system(end_animation),
        );

        app.add_system_set(
            SystemSet::on_update(GameState(Some(Move::Static)))
                .label(Labels::Movement)
                .with_system(handle_keypress),
        );
        app.add_event::<ExitedFloorEvent>();
        app.init_resource::<Events<EnteredFloorEvent>>();
        app.insert_resource(AnimationTimer(Timer::from_seconds(
            MOVE_ANIMATION_TIME,
            TimerMode::Once,
        )));
    }
}

fn continue_animation(
    mut app_state: ResMut<State<GameState>>,
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
        app_state
            .set(GameState(Some(Move::Static)))
            .expect("Could not correctly finish movement animation");
    }
}
