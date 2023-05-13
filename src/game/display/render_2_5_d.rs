use bevy::prelude::*;

use crate::{game::game_objects::{Wall, Position}, consts::{UPPER_HALF_OBJECT_Z_INDEX, LOWER_HALF_OBJECT_Z_INDEX}};

use super::render_entity;

pub fn render_object(
    commands: &mut Commands,
    higher_wall_image: Handle<Image>,
    lower_wall_image: Handle<Image>,
    x: i32,
    y: i32,
) -> (Entity, Entity) {
    let entity1 = render_entity(
            Wall,
            commands,
            higher_wall_image,
            Position {
                x,
                y,
            },
            UPPER_HALF_OBJECT_Z_INDEX,
        );
    let entity2 = render_entity(
            Wall,
            commands,
            lower_wall_image,
            Position {
                x,
                y,
            },
            LOWER_HALF_OBJECT_Z_INDEX,
        );
    (entity1, entity2)
}