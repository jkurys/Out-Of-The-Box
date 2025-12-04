use bevy::prelude::*;

use crate::game::display::render_2_5_d::get_offsets;
use crate::game::game_objects::Direction;
use crate::game::movement::events::TeleportMessage;
use crate::{
    board::GameData,
    consts::*,
    game::{
        game_objects::Floor,
        movement::{
            consts::{INTERVAL_DISTANCE_1, SPEED_1, TIME_INTERVAL_1},
            events::EnteredFloorMessage,
            resources::*,
        },
    },
};

fn animation_weight(number: f32) -> f32 {
    //this is so that the movement isn't uniform; first move with SPEED_1, and then with speed such that we move an entire tile
    if number < TIME_INTERVAL_1 {
        number * SPEED_1
    } else {
        (number - TIME_INTERVAL_1) * ((1. - INTERVAL_DISTANCE_1) / (1. - TIME_INTERVAL_1))
            + INTERVAL_DISTANCE_1
    }
}

fn modify_transform(
    mut transform: Mut<Transform>,
    direction: Direction,
    timer: &ResMut<AnimationTimer>,
    starting_x: f32,
    starting_y: f32,
    z_mod: f32,
    floor: Floor,
) {
    let distance = if floor == Floor::Ice {
        // on ice we want to have uniform movement animation
        timer.0.fraction()
    } else {
        animation_weight(timer.0.fraction())
    };
    transform.translation.z += z_mod;
    match direction {
        Direction::South => {
            transform.translation.y = starting_y - (distance - 1.) * TILE_HEIGHT;
            transform.translation.x = starting_x - (distance - 1.) * TILE_WIDTH * (101. / 300.);
        }
        Direction::North => {
            transform.translation.y = starting_y + (distance - 1.) * TILE_HEIGHT;
            transform.translation.x = starting_x + (distance - 1.) * TILE_WIDTH * (101. / 300.);
        }
        Direction::Left => {
            transform.translation.x = starting_x - (distance - 1.) * TILE_WIDTH;
        }
        Direction::Right => {
            transform.translation.x = starting_x + (distance - 1.) * TILE_WIDTH;
        }
        Direction::Up => {
            transform.translation.y = starting_y + (distance - 1.) * TILE_FRONT_HEIGHT;
        }
        Direction::Down => {
            transform.translation.y = starting_y - (distance - 1.) * TILE_FRONT_HEIGHT;
        }
    }
}

fn get_z_mod(direction: Direction) -> f32 {
    match direction {
        Direction::Up => {
            return -0.005;
        }
        Direction::Down => {
            return 0.005;
        }
        Direction::North => {
            return 0.005;
        }
        Direction::South => {
            return -0.005;
        }
        Direction::Left => {
            return 0.005;
        }
        Direction::Right => {
            return -0.005;
        }
    }
}

pub fn move_event(
    game_data: &mut ResMut<GameData>,
    event: &EnteredFloorMessage,
    query: &mut Query<&mut Transform>,
    timer: &mut ResMut<AnimationTimer>,
    is_first: bool,
) {
    let entity_storage = &mut game_data.entity_storage;
    let (position, direction) = (event.position, event.direction);
    let entity_opt = entity_storage.get(event.position);
    if let Some([higher_entities, lower_entities, side_entities]) = entity_opt {
        for &higher_entity in higher_entities.iter() {
            if let Ok(higher_transform) = query.get_mut(higher_entity) {
                let ((x, y, _), _, _) = get_offsets(position.x, position.y, position.z, 0.);
                let z_mod = if is_first { get_z_mod(direction) } else { 0. };
                modify_transform(higher_transform, direction, timer, x, y, z_mod, event.floor);
            }
        }
        for &lower_entity in lower_entities.iter() {
            if let Ok(lower_transform) = query.get_mut(lower_entity) {
                let (_, (x2, y2, _), _) = get_offsets(position.x, position.y, position.z, 0.1);
                let z_mod = if is_first { get_z_mod(direction) } else { 0. };
                modify_transform(
                    lower_transform,
                    direction,
                    timer,
                    x2,
                    y2,
                    z_mod,
                    event.floor,
                );
            }
        }
        for &side_entity in side_entities.iter() {
            if let Ok(side_transform) = query.get_mut(side_entity) {
                let ((x, y, _), _, _) = get_offsets(position.x, position.y, position.z, 0.);
                let z_mod = if is_first { get_z_mod(direction) } else { 0. };
                modify_transform(side_transform, direction, timer, x, y, z_mod, event.floor);
            }
        }
    } else {
        error!("No entities found for position {:?}", position);
    }
}

pub fn move_animation(
    time: Res<Time>,
    mut moved: MessageReader<EnteredFloorMessage>,
    mut query: Query<&mut Transform>,
    mut timer: ResMut<AnimationTimer>,
    mut game_data: ResMut<GameData>,
    mut events: Local<Vec<EnteredFloorMessage>>,
) {
    timer.0.tick(time.delta());
    if !moved.is_empty() {
        events.clear();
        for event in moved.read() {
            move_event(&mut game_data, event, &mut query, &mut timer, true);
            events.push(*event);
        }
    } else {
        for event in &events {
            move_event(&mut game_data, event, &mut query, &mut timer, false);
        }
    }
}

pub fn teleport_frame(
    event: &TeleportMessage,
    game_data: &Res<GameData>,
    query: &mut Query<&mut Sprite>,
    timer: &ResMut<AnimationTimer>,
    is_first: bool,
) {
    let entity_storage = &game_data.entity_storage;
    let entity_opt1 = entity_storage.get(event.position1);
    if let Some([mut entities1, mut lower_entities, mut side_entities]) = entity_opt1 {
        entities1.append(&mut lower_entities);
        entities1.append(&mut side_entities);
        for &entity in entities1.iter() {
            if let Ok(mut sprite) = query.get_mut(entity) {
                if is_first {
                    sprite.color.set_alpha(1. - timer.0.fraction());
                } else {
                    sprite.color.set_alpha(timer.0.fraction());
                }
            }
        }
    } else {
        error!(
            "No entities found for teleport position1 {:?}",
            event.position1
        );
    }
    let entity_opt2 = entity_storage.get(event.position2);
    if let Some([mut entities2, mut lower_entities2, mut side_entities2]) = entity_opt2 {
        entities2.append(&mut lower_entities2);
        entities2.append(&mut side_entities2);

        for &entity in entities2.iter() {
            if let Ok(mut sprite) = query.get_mut(entity) {
                if is_first {
                    sprite.color.set_alpha(1. - timer.0.fraction());
                } else {
                    sprite.color.set_alpha(timer.0.fraction());
                }
            }
        }
    } else {
        error!(
            "No entities found for teleport position2 {:?}",
            event.position2
        );
    }
}

pub fn teleport_animation(
    time: Res<Time>,
    mut reader: MessageReader<TeleportMessage>,
    mut query: Query<&mut Sprite>,
    mut timer: ResMut<AnimationTimer>,
    game_data: Res<GameData>,
    mut events: Local<Vec<TeleportMessage>>,
    mut teleport_pos: ResMut<TeleportPositions>,
    is_first: Res<TeleportFirst>,
) {
    timer.0.tick(time.delta());
    if !reader.is_empty() {
        events.clear();
        for event in reader.read() {
            teleport_frame(event, &game_data, &mut query, &timer, is_first.0);
            teleport_pos.0 = Some([event.position1, event.position2]);
            events.push(*event);
        }
    } else {
        for event in &events {
            teleport_frame(event, &game_data, &mut query, &timer, is_first.0);
        }
    }
}
