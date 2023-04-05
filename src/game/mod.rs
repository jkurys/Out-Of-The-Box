use self::resources::{BoardStates, VictoryTimer};
use crate::consts::MOVE_ANIMATION_TIME;
use crate::exit::handle_esc;
use crate::resources::Board;
use crate::utils::delete_all_components;
use crate::{
    consts::INITIAL_MAP,
    labels::Labels,
    state::{CurrentMap, DisplayState, GameState, Move},
};
use bevy::prelude::*;
use maps::load_starting_map;
use restart::{handle_restart, handle_undo};
use victory::{handle_win, handle_win_click, setup_win};

use self::victory::VictoryItem;

pub mod display;
pub mod game_objects;
mod maps;
pub mod movement;
mod resources;
mod restart;
mod victory;

#[derive(Component)]
pub struct GameItem;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(DisplayState::Game)
                .with_system(load_starting_map)
                .with_system(set_game_state),
        );
        app.add_system_set(
            SystemSet::on_update(DisplayState::Game)
                .with_system(handle_esc)
                .with_system(handle_win),
        );

        app.add_system_set(
            SystemSet::on_exit(DisplayState::Game)
                .label(Labels::ExitGame)
                .with_system(reset_game_state)
                .with_system(clear_board),
        );

        app.add_system_set(SystemSet::on_enter(DisplayState::Victory).with_system(setup_win))
            .add_system_set(
                SystemSet::on_update(DisplayState::Victory).with_system(handle_win_click),
            )
            .add_system_set(
                SystemSet::on_exit(DisplayState::Victory)
                    .with_system(delete_all_components::<VictoryItem>),
            );

        app.add_system_set(
            SystemSet::on_update(GameState(Some(Move::Static)))
                .with_system(handle_restart)
                .with_system(handle_undo),
        );
        app.insert_resource(Board::new())
            .insert_resource(BoardStates { boards: Vec::new() });
        app.insert_resource(VictoryTimer(Timer::from_seconds(
            MOVE_ANIMATION_TIME * 2.,
            TimerMode::Once,
        )));
    }
}

fn set_game_state(
    mut game_state: ResMut<State<GameState>>,
    mut current_map: ResMut<State<CurrentMap>>,
) {
    if game_state.current() != &GameState(Some(Move::Static)) {
        game_state
            .set(GameState(Some(Move::Static)))
            .expect("Could not set static game state");
    }
    if current_map.current() != &CurrentMap(Some(INITIAL_MAP)) {
        current_map
            .set(CurrentMap(Some(INITIAL_MAP)))
            .expect("Could not set initial map");
    }
}

pub fn reset_game_state(
    mut game_state: ResMut<State<GameState>>,
    mut current_map: ResMut<State<CurrentMap>>,
) {
    if game_state.current() != &GameState(None) {
        game_state
            .overwrite_set(GameState(None))
            .expect("Could not reset game state");
    }
    if current_map.current() != &CurrentMap(None) {
        current_map
            .overwrite_set(CurrentMap(None))
            .expect("Could not reset game state");
    }
}

pub fn clear_board(mut board: ResMut<Board>, mut boards: ResMut<BoardStates>) {
    board.clear();
    boards.boards.clear();
}
