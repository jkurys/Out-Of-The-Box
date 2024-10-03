use bevy::prelude::*;

use crate::game::movement::resources::{TeleportPositions, TeleportFirst};
use crate::{game::movement::resources::AnimationTimer, state::MoveState, board::Board};
use crate::game::game_objects::{*, Direction};

pub fn continue_animation(
    mut app_state: ResMut<NextState<MoveState>>,
    mut timer: ResMut<AnimationTimer>,
) {
    if !timer.0.finished() {
        return;
    }
    timer.0.reset();
    app_state.set(MoveState::AfterAnimationCalc);
}

pub fn continue_teleport_animation(
    mut app_state: ResMut<NextState<MoveState>>,
    mut timer: ResMut<AnimationTimer>,
    mut board: ResMut<Board>,
    teleport_pos: Res<TeleportPositions>,
    mut is_first: ResMut<TeleportFirst>,
) {
    if !timer.0.finished() {
        return;
    }
    timer.0.reset();
    if !is_first.0 {
        is_first.0 = true;
        app_state.set(MoveState::AfterAnimationCalc);
    } else {
        let [position1, position2] = teleport_pos.0.unwrap();
        is_first.0 = false;
        board.delete_object(position1);
        board.delete_object(position2);
        board.insert_object(position1, GameObject::Player { powerup: Some(PowerUpType::Teleport), direction: Direction::South });
        board.insert_object(position2, GameObject::TeleBox);
        app_state.set(MoveState::ReRender);
    }
}

pub fn end_rerender(
    mut app_state: ResMut<NextState<MoveState>>,
    mut query: Query<&mut Sprite>,
    board: Res<Board>,
    teleport_pos: Res<TeleportPositions>,
) {
    app_state.set(MoveState::TeleportAnimation);
    let [pos1, pos2] = teleport_pos.0.unwrap();
    let [mut entities, mut e12, mut e13] = board.get_entities(pos1).unwrap();
    let [mut e21, mut e22, mut e23] = board.get_entities(pos2).unwrap();
    entities.append(&mut e12);
    entities.append(&mut e13);
    entities.append(&mut e21);
    entities.append(&mut e22);
    entities.append(&mut e23);
    for &entity in entities.iter() {
        if let Ok(mut sprite) = query.get_mut(entity) {
            sprite.color.set_alpha(0.);
        }
    }
}
