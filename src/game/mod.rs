use self::resources::{BoardStates, VictoryTimer};
use crate::board::Board;
use crate::consts::MOVE_ANIMATION_TIME;
use crate::exit::handle_esc;
use crate::utils::delete_all_components;
use crate::{
    consts::INITIAL_MAP,
    state::{DisplayState, MoveState},
};
use bevy::prelude::*;
use maps::load_starting_map;
use restart::{handle_restart, handle_undo};
use victory::{handle_win, handle_win_click, setup_win};

use self::victory::VictoryItem;

pub mod display;
pub mod game_objects;
pub mod maps;
pub mod movement;
mod resources;
mod restart;
mod victory;

#[derive(Component)]
pub struct GameItem;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(DisplayState::Game),
            (load_starting_map, set_game_state),
        )
        .add_systems(
            Update,
            (handle_esc, handle_undo, handle_win, handle_restart)
                .run_if(in_state(DisplayState::Game)),
        )
        .add_systems(OnExit(DisplayState::Game), clear_board);

        app.add_systems(OnEnter(DisplayState::Victory), setup_win)
            .add_systems(
                Update,
                handle_win_click.run_if(in_state(DisplayState::Victory)),
            )
            .add_systems(
                OnExit(DisplayState::Victory),
                delete_all_components::<VictoryItem>,
            );

        app.insert_resource(Board::new())
            .insert_resource(BoardStates { boards: Vec::new() });
        app.insert_resource(VictoryTimer(Timer::from_seconds(
            MOVE_ANIMATION_TIME * 2.,
            TimerMode::Once,
        )));
    }
}

fn set_game_state(mut game_state: ResMut<NextState<MoveState>>, mut board: ResMut<Board>) {
    game_state.set(MoveState::Static);
    board.set_current_map(INITIAL_MAP);
}

pub fn clear_board(mut board: ResMut<Board>, mut boards: ResMut<BoardStates>) {
    board.clear();
    boards.boards.clear();
}
