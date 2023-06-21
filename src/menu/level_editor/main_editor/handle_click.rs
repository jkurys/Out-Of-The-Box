use bevy::prelude::*;

use crate::{
    board::Board,
    components::GameEntity,
    consts::{TILE_TEXTURE, TURTLE_TEXTURES},
    game::game_objects::{Direction, Floor, GameObject},
};

use super::LevelEditorChangable;

pub fn handle_level_editor_click(
    mut changable_query: Query<
        (&LevelEditorChangable, &Interaction, &mut UiImage),
        With<LevelEditorChangable>,
    >,
    mut clickable_query: Query<
        (&Interaction, &UiImage, &GameEntity, &mut BackgroundColor),
        Without<LevelEditorChangable>,
    >,
    mouse: Res<Input<MouseButton>>,
    mut board: ResMut<Board>,
    mut current_object: Local<GameEntity>,
    mut image: Local<(UiImage, bool)>,
    input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    for (changable, interaction, mut new_image) in changable_query.iter_mut() {
        let position = changable.0;
        if *interaction == Interaction::Clicked {
            if image.1 {
                *new_image = image.0.clone();
                board.insert(position, *current_object);
                if let GameEntity::Object(GameObject::HidingWall { color }) = *current_object {
                    board.insert_floor(
                        position,
                        Floor::HiddenWall {
                            hidden_by_default: false,
                            color,
                        },
                    );
                }
            }
        }
        if *interaction == Interaction::Hovered && mouse.just_pressed(MouseButton::Right) {
            board.delete_object(position);
            board.delete_floor(position);
            *new_image = asset_server.load(TILE_TEXTURE).into();
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
    if let GameEntity::Object(GameObject::Turtle {
        direction: _,
        color,
    }) = *current_object
    {
        let mut maybe_dir = None;
        if input.just_pressed(KeyCode::Up) {
            maybe_dir = Some(Direction::Up);
        } else if input.just_pressed(KeyCode::Down) {
            maybe_dir = Some(Direction::Down);
        } else if input.just_pressed(KeyCode::Left) {
            maybe_dir = Some(Direction::Left);
        } else if input.just_pressed(KeyCode::Right) {
            maybe_dir = Some(Direction::Right);
        }
        if let Some(dir) = maybe_dir {
            image.0 = asset_server.load(TURTLE_TEXTURES[dir.to_num()]).into();
            *current_object = GameEntity::Object(GameObject::Turtle {
                direction: dir,
                color,
            });
        }
    }
}
