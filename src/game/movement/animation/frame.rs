use bevy::prelude::*;

use crate::game::display::render_2_5_d::get_offsets;
use crate::game::game_objects::Direction;
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
