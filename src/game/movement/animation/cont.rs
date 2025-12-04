use bevy::prelude::*;

use crate::board::GameData;
use crate::game::movement::resources::{TeleportFirst, TeleportPositions};
use crate::{game::movement::resources::AnimationTimer, state::MoveState};

pub fn continue_animation(
    mut app_state: ResMut<NextState<MoveState>>,
    mut timer: ResMut<AnimationTimer>,
) {
    if !timer.0.is_finished() {
        return;
    }
    timer.0.reset();
    app_state.set(MoveState::AfterAnimationCalc);
}

pub fn continue_teleport_animation(
    mut app_state: ResMut<NextState<MoveState>>,
    mut timer: ResMut<AnimationTimer>,
    mut game_data: ResMut<GameData>,
    teleport_pos: Res<TeleportPositions>,
    mut is_first: ResMut<TeleportFirst>,
) {
    let GameData {
        board,
        entity_storage,
    } = &mut *game_data;
    if !timer.0.is_finished() {
        return;
    }
    timer.0.reset();
    if !is_first.0 {
        is_first.0 = true;
        app_state.set(MoveState::AfterAnimationCalc);
    } else {
        let [position1, position2] = teleport_pos.0.unwrap();
        is_first.0 = false;
        let obj1 = board.get_object_type(position1);
        let entities1_opt = entity_storage.get(position1);
        let obj2 = board.get_object_type(position2);
        let entities2_opt = entity_storage.get(position2);
        board.delete_object(position1, entity_storage);
        board.delete_object(position2, entity_storage);
        board.insert_object(position1, obj2);
        if let Some(entities2) = entities2_opt {
            entity_storage.insert_entities(position1, entities2);
        }
        board.insert_object(position2, obj1);
        if let Some(entities1) = entities1_opt {
            entity_storage.insert_entities(position2, entities1);
        }
        app_state.set(MoveState::ReRender);
    }
}

pub fn end_rerender(
    mut app_state: ResMut<NextState<MoveState>>,
    mut query: Query<&mut Sprite>,
    game_data: Res<GameData>,
    teleport_pos: Res<TeleportPositions>,
) {
    let entity_storage = &game_data.entity_storage;
    app_state.set(MoveState::TeleportAnimation);
    let [pos1, pos2] = teleport_pos.0.unwrap();
    let entities1_opt = entity_storage.get(pos1);
    let entities2_opt = entity_storage.get(pos2);
    let mut entities = Vec::new();
    if let Some([mut e11, mut e12, mut e13]) = entities1_opt {
        entities.append(&mut e11);
        entities.append(&mut e12);
        entities.append(&mut e13);
    }
    if let Some([mut e21, mut e22, mut e23]) = entities2_opt {
        entities.append(&mut e21);
        entities.append(&mut e22);
        entities.append(&mut e23);
    }
    for &entity in entities.iter() {
        if let Ok(mut sprite) = query.get_mut(entity) {
            sprite.color.set_alpha(0.);
        }
    }
}
