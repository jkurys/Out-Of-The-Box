use bevy::prelude::*;

use crate::consts::*;
use crate::game::game_objects::{Direction, *};
use crate::resources::Images;

use super::render_2_5_d::render_sticker;
use crate::board::GameData;

fn are_both_turtles(obj1: GameObject, obj2: GameObject) -> bool {
    (matches!(
        obj1,
        GameObject::Turtle {
            direction: _,
            color: _
        } | GameObject::TurtleHead {
            direction: _,
            color: _
        }
    ) && matches!(
        obj2,
        GameObject::TurtleHead {
            direction: _,
            color: _
        }
    )) || (matches!(
        obj2,
        GameObject::Turtle {
            direction: _,
            color: _
        } | GameObject::TurtleHead {
            direction: _,
            color: _
        }
    ) && matches!(
        obj1,
        GameObject::TurtleHead {
            direction: _,
            color: _
        }
    ))
}

pub fn render_glue(
    position: Position,
    game_data: &mut ResMut<GameData>,
    images: &Res<Images>,
    mut commands: &mut Commands,
) {
    let GameData {
        board,
        entity_storage,
    } = &mut **game_data;
    let Position { x, y, z } = position;
    let block = board.get_block(position);
    let right_pos = position.next_position(Direction::Right);
    let left_pos = position.next_position(Direction::Left);
    let up_pos = position.next_position(Direction::North);
    let down_pos = position.next_position(Direction::South);

    let object = board.get_object_type(position);
    let right_object = board.get_object_type(right_pos);
    let left_object = board.get_object_type(left_pos);
    let up_object = board.get_object_type(up_pos);
    let down_object = board.get_object_type(down_pos);

    if block.contains_position(right_pos) {
        let images_opt = if object == GameObject::Box && right_object == GameObject::Box {
            images.glue_images.clone()
        } else if are_both_turtles(object, right_object) || object == GameObject::Empty {
            None
        } else {
            images.glue_images.clone()
        };
        if let Some(images) = images_opt {
            let glue_entity = render_sticker(
                &mut commands,
                0,
                x,
                y,
                z,
                images,
                Glue,
                UPPER_HALF_STICKER_Z_INDEX,
            );
            entity_storage.append_entities(position, [vec![glue_entity], Vec::new(), Vec::new()]);
        }
    }
    if block.contains_position(left_pos) {
        let images_opt = if board.get_object_type(position) == GameObject::Box
            && board.get_object_type(left_pos) == GameObject::Box
        {
            images.glue_images.clone()
        } else if are_both_turtles(object, left_object) || object == GameObject::Empty {
            None
        } else {
            images.glue_images.clone()
        };
        if let Some(images) = images_opt {
            let glue_entity = render_sticker(
                &mut commands,
                1,
                x,
                y,
                z,
                images,
                Glue,
                UPPER_HALF_STICKER_Z_INDEX,
            );
            entity_storage.append_entities(position, [vec![glue_entity], Vec::new(), Vec::new()]);
        }
    }
    if block.contains_position(up_pos) {
        let images_opt = if board.get_object_type(position) == GameObject::Box
            && board.get_object_type(up_pos) == GameObject::Box
        {
            images.glue_images.clone()
        } else if are_both_turtles(object, up_object) || object == GameObject::Empty {
            None
        } else {
            images.glue_images.clone()
        };
        if let Some(images) = images_opt {
            let glue_entity = render_sticker(
                &mut commands,
                2,
                x,
                y,
                z,
                images,
                Glue,
                UPPER_HALF_STICKER_Z_INDEX,
            );
            entity_storage.append_entities(position, [vec![glue_entity], Vec::new(), Vec::new()]);
        }
    }
    if block.contains_position(down_pos) {
        let images_opt = if board.get_object_type(position) == GameObject::Box
            && board.get_object_type(down_pos) == GameObject::Box
        {
            images.glue_images.clone()
        } else if are_both_turtles(object, down_object) || object == GameObject::Empty {
            None
        } else {
            images.glue_images.clone()
        };
        if let Some(images) = images_opt {
            let glue_entity = render_sticker(
                &mut commands,
                3,
                x,
                y,
                z,
                images,
                Glue,
                UPPER_HALF_STICKER_Z_INDEX,
            );
            entity_storage.append_entities(position, [vec![glue_entity], Vec::new(), Vec::new()]);
        }
    }
}
