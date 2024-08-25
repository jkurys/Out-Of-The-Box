use bevy::prelude::*;

use crate::{
    board::Board,
    components::GameEntity,
    consts::{
        BOX_TEXTURE, HIDDEN_WALL_TEXTURES, MAX_MAPS, PLAYER_TEXTURES, PLUS_TEXTURE,
        SHOWN_HIDDEN_WALL_TEXTURES, STICKER_TEXTURES, TURTLE_TEXTURES, WALL_TEXTURE, BUTTON_TEXTURES,
    },
    game::game_objects::{Direction, Floor, GameObject},
    menu::level_editor::{
        resources::BoardSize,
        utils::{spawn_small_button, spawn_small_button_with_sticker},
        LevelEditorItem,
    },
    resources::{CurrentSprite, Images},
};

use super::{LevelEditorTab, LevelEditorTabPlus};

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
    let water_image = asset_server.load("textures/water.png");
    let ice_image = asset_server.load("textures/ice.png");
    let sticker_images = STICKER_TEXTURES.map(|texture| asset_server.load(texture));
    let hidden_wall_images = SHOWN_HIDDEN_WALL_TEXTURES.map(|texture| asset_server.load(texture));
    let bottom_hidden_wall_images = HIDDEN_WALL_TEXTURES.map(|texture| asset_server.load(texture));
    let player_image = asset_server.load(PLAYER_TEXTURES[current_sprite.0]);
    let plus_image = asset_server.load(PLUS_TEXTURE);
    let button_images = [asset_server.load(BUTTON_TEXTURES[0]), asset_server.load(BUTTON_TEXTURES[1]), asset_server.load(BUTTON_TEXTURES[2])];
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::DARK_GREEN),
            visibility: Visibility::Visible,
            style: Style {
                width: Val::Percent(5.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                align_self: AlignSelf::End,
                ..default()
            },
            ..default()
        })
        .insert(LevelEditorItem)
        .with_children(|parent| {
            spawn_small_button(
                parent,
                box_image.clone(),
                GameEntity::Object(GameObject::Box),
            );
            for (color, image) in hidden_wall_images.iter().enumerate() {
                spawn_small_button(
                    parent,
                    image.clone(),
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
            style: Style {
                width: Val::Percent(5.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                align_self: AlignSelf::Start,
                ..default()
            },
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
                images.dirt_image.clone(),
                GameEntity::Floor(Floor::Dirt),
            );
            spawn_small_button(
                parent,
                water_image,
                GameEntity::Floor(Floor::Void),
            );
            spawn_small_button(
                parent,
                ice_image,
                GameEntity::Floor(Floor::Ice),
            );
            for (color, image) in button_images.iter().enumerate() {
                spawn_small_button(
                    parent,
                    image.clone(),
                    GameEntity::Floor(Floor::Button(color)),
                );
            }
            for (color, image) in bottom_hidden_wall_images.iter().enumerate() {
                spawn_small_button(
                    parent,
                    image.clone(),
                    GameEntity::Floor(Floor::HiddenWall {
                        hidden_by_default: true,
                        color,
                    }),
                );
            }
            for (color, image) in sticker_images.iter().enumerate() {
                spawn_small_button_with_sticker(
                    parent,
                    turtle_image.clone(),
                    GameEntity::Object(GameObject::Turtle {
                        color,
                        direction: Direction::Left,
                    }),
                    image.clone(),
                );
            }
            spawn_small_button(
                parent,
                images.tile_image.clone(),
                GameEntity::Floor(Floor::Tile),
            );
            // spawn_small_button(
            //     parent,
            //     images.warp_image.clone(),
            //     GameEntity::Floor(Floor::Warp(1)),
            // );
        });
    commands
        .spawn(NodeBundle {
            background_color: Color::BLUE.into(),
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(3.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                align_self: AlignSelf::Start,
                ..default()
            },
            ..default()
        })
        .insert(LevelEditorItem)
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Percent(10.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    ..default()
                })
                .insert(LevelEditorTab(0));
            for i in 1..MAX_MAPS {
                parent
                    .spawn(ButtonBundle {
                        visibility: Visibility::Hidden,
                        style: Style {
                            width: Val::Percent(10.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(LevelEditorTab(i));
            }
            parent
                .spawn(ImageBundle {
                    image: plus_image.into(),
                    ..default()
                })
                .insert((LevelEditorTabPlus, ButtonBundle::default()));
        });
}
