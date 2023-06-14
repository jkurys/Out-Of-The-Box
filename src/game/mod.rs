use self::resources::{BoardStates, VictoryTimer};
use crate::consts::MOVE_ANIMATION_TIME;
use crate::exit::handle_esc;
use crate::board::Board;
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
mod maps;
mod resources;
pub mod movement;
mod restart;
mod victory;

#[derive(Component)]
pub struct GameItem;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((load_starting_map, set_game_state).in_schedule(OnEnter(DisplayState::Game)))
            .add_systems((
                handle_esc,
                handle_undo,
                handle_win,
                handle_restart,
            ).in_set(OnUpdate(DisplayState::Game)))
            .add_system(clear_board.in_schedule(OnExit(DisplayState::Game)));

        app.add_system(setup_win.in_schedule(OnEnter(DisplayState::Victory)))
            .add_system(handle_win_click.in_set(OnUpdate(DisplayState::Victory)))
            .add_system(delete_all_components::<VictoryItem>.in_schedule(OnExit(DisplayState::Victory)));

        app.insert_resource(Board::new())
            .insert_resource(BoardStates { boards: Vec::new() });
        app.insert_resource(VictoryTimer(Timer::from_seconds(
            MOVE_ANIMATION_TIME * 2.,
            TimerMode::Once,
        )));
    }
}

fn set_game_state(
    mut game_state: ResMut<NextState<MoveState>>,
    mut board: ResMut<Board>,
) {
    game_state.set(MoveState::Static);
    board.set_current_map(INITIAL_MAP);
}

// pub fn reset_game_state(
//     mut game_state: ResMut<State<MoveState>>,
//     mut board: ResMut<Board>,
// ) {
//     // TRZEBA DAĆ CONDITION NA WYKONYWANIE TYCH RZECZY TYLKO JESLI JESTESMY W GAME
// }

pub fn clear_board(mut board: ResMut<Board>, mut boards: ResMut<BoardStates>) {
    board.clear();
    boards.boards.clear();
}
