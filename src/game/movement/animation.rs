use super::{events::EnteredFloorEvent, resources::AnimationTimer};
use crate::{consts::TILE_SIZE, board::Board};
use bevy::prelude::*;

use crate::game::game_objects::{Direction, *};

use super::{
    consts::{INTERVAL_DISTANCE_1, SPEED_1, TIME_INTERVAL_1},
    MovableInQuery,
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
    floor: Floor,
) {
    let distance = if floor == Floor::Ice {
        // on ice we want to have uniform movement animation
        timer.0.percent()
    } else {
        animation_weight(timer.0.percent())
    };
    match direction {
        Direction::Down => {
            transform.translation.y = starting_y - (distance - 1.) * TILE_SIZE;
        }
        Direction::Up => {
            transform.translation.y = starting_y + (distance - 1.) * TILE_SIZE;
        }
        Direction::Left => {
            transform.translation.x = starting_x - (distance - 1.) * TILE_SIZE;
        }
        Direction::Right => {
            transform.translation.x = starting_x + (distance - 1.) * TILE_SIZE;
        }
    }
}

pub fn move_animation(
    time: Res<Time>,
    mut moved: EventReader<EnteredFloorEvent>,
    mut query: Query<&mut Transform, MovableInQuery>,
    mut timer: ResMut<AnimationTimer>,
    board: Res<Board>,
    mut events: Local<Vec<EnteredFloorEvent>>,
) {
    timer.0.tick(time.delta());
    if !moved.is_empty() {
        events.clear();
        for event in moved.iter() {
            events.push(*event);
            let (position, direction) = (event.position, event.direction);
            let entity_opt = board.get_entities(event.position);
            if let Some([higher_entity, lower_entity]) = entity_opt {
                
                let higher_transform = query.get_mut(higher_entity).expect("Moved box entity not found");
                let (x, y) = (position.x as f32 * TILE_SIZE, (position.y as f32 + 0.25) * TILE_SIZE);
                modify_transform(
                    higher_transform,
                    direction,
                    &timer,
                    x,
                    y,
                    event.floor,
                );
                let lower_transform = query.get_mut(lower_entity).expect("Moved box entity not found");
                let (x2, y2) = (position.x as f32 * TILE_SIZE, (position.y as f32 - 0.375) * TILE_SIZE);
                modify_transform(
                    lower_transform,
                    direction,
                    &timer,
                    x2,
                    y2,
                    event.floor,
                );
            }
        }
    } else {
        for event in &events {
            let (position, direction) = (event.position, event.direction);
            let entity_opt = board.get_entities(event.position);
            if let Some([higher_entity, lower_entity]) = entity_opt {
                let higher_transform = query.get_mut(higher_entity).expect("Moved box entity not found");
                let (x, y) = (position.x as f32 * TILE_SIZE, (position.y as f32 + 0.25) * TILE_SIZE);
                modify_transform(
                    higher_transform,
                    direction,
                    &timer,
                    x,
                    y,
                    event.floor,
                );
                let lower_transform = query.get_mut(lower_entity).expect("Moved box entity not found");
                let (x, y) = (position.x as f32 * TILE_SIZE, (position.y as f32 - 0.375) * TILE_SIZE);
                modify_transform(
                    lower_transform,
                    direction,
                    &timer,
                    x,
                    y,
                    event.floor,
                );
            }
        }
    }
}

pub fn end_animation(mut timer: ResMut<AnimationTimer>) {
    timer.0.reset();
}
