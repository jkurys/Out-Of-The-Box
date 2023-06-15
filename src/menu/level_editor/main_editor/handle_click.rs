use bevy::prelude::*;

use crate::{board::Board, game::game_objects::{GameObject, Floor}, components::GameEntity};

use super::LevelEditorChangable;

pub fn handle_level_editor_click(
    mut changable_query: Query<
        (&LevelEditorChangable, &Interaction, &mut UiImage),
        With<LevelEditorChangable>,
    >,
    mut clickable_query: Query<(&Interaction, &UiImage, &GameEntity, &mut BackgroundColor), Without<LevelEditorChangable>>,
    mut board: ResMut<Board>,
    mut current_object: Local<GameEntity>,
    mut image: Local<(UiImage, bool)>,
) {
    for (changable, interaction, mut new_image) in changable_query.iter_mut() {
        let position = changable.0;
        if *interaction == Interaction::Clicked {
            if image.1 {
                *new_image = image.0.clone();
                board.insert(position, *current_object);
                if let GameEntity::Object(GameObject::HidingWall { color }) = *current_object {
                    board.insert_floor(position, Floor::HiddenWall { hidden_by_default: false, color });
                }
            }
        }
    }
    for (&interaction, new_image, object_or_floor, mut color) in clickable_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                image.0 = new_image.clone();
                image.1 = true;
                *current_object = *object_or_floor;
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.7, 0.7, 0.7).into();
            }
            Interaction::None => {
                *color = Color::WHITE.into();
            }
        }
    }
}