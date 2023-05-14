use bevy::prelude::*;

use crate::{game::game_objects::Position, consts::{UPPER_HALF_OBJECT_Z_INDEX, LOWER_HALF_OBJECT_Z_INDEX}};

use super::render_entity;

pub fn render_object<T>(
    commands: &mut Commands,
    higher_image: Handle<Image>,
    lower_image: Handle<Image>,
    x: i32,
    y: i32,
    component: T,
) -> [Entity; 2] where 
    T: Component + Clone,
{
    let entity1 = render_entity(
            component.clone(),
            commands,
            higher_image,
            Position {
                x,
                y,
            },
            UPPER_HALF_OBJECT_Z_INDEX,
        );
    let entity2 = render_entity(
            component.clone(),
            commands,
            lower_image,
            Position {
                x,
                y,
            },
            LOWER_HALF_OBJECT_Z_INDEX,
        );
    [entity1, entity2]
}