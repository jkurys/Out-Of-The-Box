use bevy::prelude::*;

use crate::game::display::render_2_5_d::get_offsets;
use crate::game::game_objects::Direction;
use crate::game::movement::events::TeleportEvent;
use crate::{
    board::Board,
    consts::*,
    game::{
        game_objects::Floor,
        movement::{
            consts::{INTERVAL_DISTANCE_1, SPEED_1, TIME_INTERVAL_1},
            events::EnteredFloorEvent,
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
            transform.translation.x = starting_x - (distance - 1.) * TILE_WIDTH * (101./300.);
        }
        Direction::North => {
            transform.translation.y = starting_y + (distance - 1.) * TILE_HEIGHT;
            transform.translation.x = starting_x + (distance - 1.) * TILE_WIDTH * (101./300.);
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

fn get_z_mod(
    direction: Direction,
) -> f32 {
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
    board: &Res<Board>,
    event: &EnteredFloorEvent,
    query: &mut Query<&mut Transform>,
    timer: &mut ResMut<AnimationTimer>,
    is_first: bool,
) {
    let (position, direction) = (event.position, event.direction);
    let entity_opt = board.get_entities(event.position);
    if let Some([higher_entities, lower_entities, side_entities]) = entity_opt {
        for &higher_entity in higher_entities.iter() {
            if let Ok(higher_transform) = query.get_mut(higher_entity) {
                let ((x, y, _), _, _) = get_offsets(position.x, position.y, position.z, 0.);
                let z_mod = if is_first {
                    get_z_mod(direction)
                } else {
                    0.
                };
                modify_transform(higher_transform, direction, timer, x, y, z_mod, event.floor);
            }
        }
        for &lower_entity in lower_entities.iter() {
            if let Ok(lower_transform) = query.get_mut(lower_entity) {
                let (_, (x2, y2, _), _) = get_offsets(position.x, position.y, position.z, 0.1);
                let z_mod = if is_first {
                    get_z_mod(direction)
                } else {
                    0.
                };
                modify_transform(lower_transform, direction, timer, x2, y2, z_mod, event.floor);
            }
        }
        for &side_entity in side_entities.iter() {
            if let Ok(side_transform) = query.get_mut(side_entity) {
                let ((x, y, _), _, _) = get_offsets(position.x, position.y, position.z, 0.);
                let z_mod = if is_first {
                    get_z_mod(direction)
                } else {
                    0.
                };
                modify_transform(side_transform, direction, timer, x, y, z_mod, event.floor);
            }
        }
    }
}

pub fn move_animation(
    time: Res<Time>,
    mut moved: EventReader<EnteredFloorEvent>,
    mut query: Query<&mut Transform>,
    mut timer: ResMut<AnimationTimer>,
    board: Res<Board>,
    mut events: Local<Vec<EnteredFloorEvent>>,
) {
    timer.0.tick(time.delta());
    if !moved.is_empty() {
        events.clear();
        for event in moved.read() {
            move_event(&board, event, &mut query, &mut timer, true);
            events.push(*event);
        }
    } else {
        for event in &events {
            move_event(&board, event, &mut query, &mut timer, false);
        }
    }
}

pub fn teleport_frame(
    event: &TeleportEvent,
    board: &mut ResMut<Board>,
    query: &mut Query<&mut Sprite>,
    timer: &ResMut<AnimationTimer>,
    is_first: bool,
) {
    let entity_opt1 = board.get_entities(event.position1);
    if let Some([mut entities1, mut lower_entities, mut side_entities]) = entity_opt1 {
        entities1.append(&mut lower_entities);
        entities1.append(&mut side_entities);
        let entity_opt2 = board.get_entities(event.position2);
        if let Some([mut entities2, mut lower_entities2, mut side_entities2]) = entity_opt2 {
            entities2.append(&mut lower_entities2);
            entities2.append(&mut side_entities2);
            for &entity in entities1.iter() {
                if let Ok(mut sprite) = query.get_mut(entity) {
                    if is_first {
                        sprite.color.set_alpha(1. - timer.0.fraction());
                    } else {
                        sprite.color.set_alpha(timer.0.fraction());
                    }
                }
            }
            for &entity in entities2.iter() {
                if let Ok(mut sprite) = query.get_mut(entity) {
                    if is_first {
                        sprite.color.set_alpha(1. - timer.0.fraction());
                    } else {
                        sprite.color.set_alpha(timer.0.fraction());
                    }
                }
            }
        }
    }
}

pub fn teleport_animation(
    time: Res<Time>,
    mut reader: EventReader<TeleportEvent>,
    mut query: Query<&mut Sprite>,
    mut timer: ResMut<AnimationTimer>,
    mut board: ResMut<Board>,
    mut events: Local<Vec<TeleportEvent>>,
    mut teleport_pos: ResMut<TeleportPositions>,
    is_first: Res<TeleportFirst>,
) {
    timer.0.tick(time.delta());
    if !reader.is_empty() {
        events.clear();
        for event in reader.read() {
            teleport_frame(event, &mut board, &mut query, &timer, is_first.0);
            teleport_pos.0 = Some([event.position1, event.position2]);
            events.push(*event);
        }
    } else {
        for event in &events {
            teleport_frame(event, &mut board, &mut query, &timer, is_first.0);
        }
    }
}
