use bevy::prelude::*;

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
            MovableInQuery,
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
            transform.translation.y = starting_y - (distance - 1.) * TILE_HEIGHT;
            transform.translation.x = starting_x - (distance - 1.) * TILE_WIDTH * (101./300.);
        }
        Direction::Up => {
            transform.translation.y = starting_y + (distance - 1.) * TILE_HEIGHT;
            transform.translation.x = starting_x + (distance - 1.) * TILE_WIDTH * (101./300.);
        }
        Direction::Left => {
            transform.translation.x = starting_x - (distance - 1.) * TILE_WIDTH;
        }
        Direction::Right => {
            transform.translation.x = starting_x + (distance - 1.) * TILE_WIDTH;
        }
    }
}
pub fn move_event(
    board: &Res<Board>,
    event: &EnteredFloorEvent,
    query: &mut Query<&mut Transform, MovableInQuery>,
    timer: &mut ResMut<AnimationTimer>,
) {
    let (position, direction) = (event.position, event.direction);
    let entity_opt = board.get_entities(event.position);
    if let Some([higher_entities, lower_entities]) = entity_opt {
        for &higher_entity in higher_entities.iter() {
            if let Ok(higher_transform) = query.get_mut(higher_entity) {
                let (x, y) = (
                    (position.x as f32) * TILE_WIDTH + (position.y as f32 * (101./300.) * TILE_WIDTH),
                    (position.y as f32 + 1.) * (TILE_HEIGHT - 3.) + ((1 - 1) as f32 * TILE_FRONT_HEIGHT), // NOTE: this 1 - 1 corresponds to z index; patch this soon
                );
                modify_transform(higher_transform, direction, timer, x, y, event.floor);
            }
        }
        for &lower_entity in lower_entities.iter() {
            if let Ok(lower_transform) = query.get_mut(lower_entity) {
                let (x2, y2) = (
                    (position.x as f32) * TILE_WIDTH + (position.y as f32 * (101./300.) * TILE_WIDTH),
                    (position.y as f32) * (TILE_HEIGHT - 3.) + ((1 - 1) as f32 * TILE_FRONT_HEIGHT),
                );
                modify_transform(lower_transform, direction, timer, x2, y2, event.floor);
            }
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
            move_event(&board, event, &mut query, &mut timer);
            events.push(*event);
        }
    } else {
        for event in &events {
            move_event(&board, event, &mut query, &mut timer);
        }
    }
}
