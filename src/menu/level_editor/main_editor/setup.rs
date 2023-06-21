use bevy::prelude::*;

use crate::{
    board::Board,
    components::GameEntity,
    consts::{
        BOX_TEXTURE, HIDDEN_WALL_TEXTURES, PLAYER_TEXTURES, SHOWN_HIDDEN_WALL_TEXTURES,
        STICKER_TEXTURES, TURTLE_TEXTURES, WALL_TEXTURE,
    },
    game::game_objects::{Direction, Floor, GameObject},
    menu::level_editor::{
        resources::BoardSize,
        utils::{spawn_small_button, spawn_small_button_with_sticker},
        LevelEditorItem,
    },
    resources::{CurrentSprite, Images},
};

use super::styles::*;

pub fn set_board_size(board_size: Res<BoardSize>, mut boards: ResMut<Board>) {
    boards.set_map_size(*board_size);
}

pub fn setup_level_editor_board(
    mut commands: Commands,
    images: Res<Images>,
    asset_server: Res<AssetServer>,
    current_sprite: Res<CurrentSprite>,
) {
    let turtle_image = asset_server.load(TURTLE_TEXTURES[0]);
    let wall_image = asset_server.load(WALL_TEXTURE);
    let box_image = asset_server.load(BOX_TEXTURE);
    let sticker_images = STICKER_TEXTURES.map(|texture| asset_server.load(texture));
    let hidden_wall_images = SHOWN_HIDDEN_WALL_TEXTURES.map(|texture| asset_server.load(texture));
    let bottom_hidden_wall_images = HIDDEN_WALL_TEXTURES.map(|texture| asset_server.load(texture));
    let player_image = asset_server.load(PLAYER_TEXTURES[current_sprite.0]);
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::DARK_GREEN),
            visibility: Visibility::Visible,
            style: OBJECTS_COMPARTMENT_STYLE,
            ..default()
        })
        .insert(LevelEditorItem)
        .with_children(|parent| {
            spawn_small_button(
                parent,
                box_image.clone(),
                GameEntity::Object(GameObject::Box),
            );
            for color in 0..3 {
                spawn_small_button(
                    parent,
                    hidden_wall_images[color].clone(),
                    GameEntity::Object(GameObject::HidingWall { color }),
                );
            }
            spawn_small_button(
                parent,
                wall_image.clone(),
                GameEntity::Object(GameObject::Wall),
            );
            spawn_small_button(
                parent,
                player_image.clone(),
                GameEntity::Object(GameObject::Player),
            );
        });
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::GREEN),
            visibility: Visibility::Visible,
            style: FLOORS_COMPARTMENT_STYLE,
            ..default()
        })
        .insert(LevelEditorItem)
        .with_children(|parent| {
            spawn_small_button(
                parent,
                images.goal_image.clone(),
                GameEntity::Floor(Floor::Goal),
            );
            spawn_small_button(
                parent,
                images.ice_image.clone(),
                GameEntity::Floor(Floor::Ice),
            );
            for color in 0..3 {
                spawn_small_button(
                    parent,
                    images.button_images[color].clone(),
                    GameEntity::Floor(Floor::Button(color)),
                );
            }
            for color in 0..3 {
                spawn_small_button(
                    parent,
                    bottom_hidden_wall_images[color].clone(),
                    GameEntity::Floor(Floor::HiddenWall {
                        hidden_by_default: true,
                        color,
                    }),
                );
            }
            for color in 0..3 {
                spawn_small_button_with_sticker(
                    parent,
                    turtle_image.clone(),
                    GameEntity::Object(GameObject::Turtle {
                        color,
                        direction: Direction::Left,
                    }),
                    sticker_images[color].clone(),
                );
            }
            spawn_small_button(
                parent,
                images.tile_image.clone(),
                GameEntity::Floor(Floor::Tile),
            );
            spawn_small_button(
                parent,
                images.warp_image.clone(),
                GameEntity::Floor(Floor::Warp(1)),
            );
        });
    commands.spawn(NodeBundle {
        background_color: Color::BLUE.into(),
        style: TABS_COMPARTMENT_STYLE,
        ..default()
    }).insert(LevelEditorItem);
}
