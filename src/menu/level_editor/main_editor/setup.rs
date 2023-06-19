use bevy::prelude::*;

use crate::{
    board::Board,
    components::GameEntity,
    consts::{
        BOX_TEXTURE, HIDDEN_WALL_TEXTURES, MAX_MAPS, PLAYER_TEXTURES, PLUS_TEXTURE,
        SHOWN_HIDDEN_WALL_TEXTURES, TURTLE_TEXTURE, WALL_TEXTURE,
    },
    game::game_objects::{Floor, GameObject, Position},
    menu::level_editor::{
        resources::BoardSize,
        utils::{spawn_small_button, spawn_small_image},
        LevelEditorItem,
    },
    resources::{CurrentSprite, Images, MapSize},
    utils::offset_coordinate,
};

use super::{styles::*, LevelEditorChangable, LevelEditorTab, LevelEditorTabPlus, LevelEditorTabs};

pub fn setup_level_editor_board(
    mut commands: Commands,
    images: Res<Images>,
    board_size: Res<BoardSize>,
    mut boards: ResMut<Board>,
    asset_server: Res<AssetServer>,
    current_sprite: Res<CurrentSprite>,
) {
    let BoardSize { width, height } = *board_size;
    boards.set_map_size(MapSize { width, height });
    let plus_image = asset_server.load(PLUS_TEXTURE);
    let bottom_border = offset_coordinate(0, height as i32);
    let top_border = offset_coordinate(height as i32 - 1, height as i32);
    let left_border = offset_coordinate(0, width as i32);
    let right_border = offset_coordinate(width as i32 - 1, width as i32);
    let turtle_image = asset_server.load(TURTLE_TEXTURE);
    let wall_image = asset_server.load(WALL_TEXTURE);
    let box_image = asset_server.load(BOX_TEXTURE);
    let hidden_wall_images = SHOWN_HIDDEN_WALL_TEXTURES.map(|texture| asset_server.load(texture));
    let bottom_hidden_wall_images = HIDDEN_WALL_TEXTURES.map(|texture| asset_server.load(texture));
    let player_image = asset_server.load(PLAYER_TEXTURES[current_sprite.0]);
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::BLACK),
            visibility: Visibility::Visible,
            style: EDITOR_STYLE,
            ..default()
        })
        .insert(LevelEditorItem)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    background_color: BackgroundColor(Color::GRAY),
                    visibility: Visibility::Visible,
                    style: BOARD_COMPARTMENT_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            background_color: BackgroundColor(Color::GRAY),
                            visibility: Visibility::Visible,
                            style: BOARD_STYLE,
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    background_color: BackgroundColor(Color::GRAY),
                                    visibility: Visibility::Visible,
                                    style: COLUMN_STYLE,
                                    ..default()
                                })
                                .with_children(|parent| {
                                    for _ in 0..height + 2 {
                                        spawn_small_image(parent, wall_image.clone());
                                    }
                                });
                            for x in left_border..(right_border + 1) {
                                parent
                                    .spawn(NodeBundle {
                                        background_color: BackgroundColor(Color::GRAY),
                                        visibility: Visibility::Visible,
                                        style: COLUMN_STYLE,
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        spawn_small_image(parent, wall_image.clone());
                                        for y in (bottom_border..=top_border).rev() {
                                            spawn_small_button(
                                                parent,
                                                images.tile_image.clone(),
                                                LevelEditorChangable(Position { x, y }),
                                            );
                                        }
                                        spawn_small_image(parent, wall_image.clone());
                                    });
                            }
                            parent
                                .spawn(NodeBundle {
                                    background_color: BackgroundColor(Color::GRAY),
                                    visibility: Visibility::Visible,
                                    style: COLUMN_STYLE,
                                    ..default()
                                })
                                .with_children(|parent| {
                                    for _ in 0..height + 2 {
                                        spawn_small_image(parent, wall_image.clone());
                                    }
                                });
                        });
                    parent
                        .spawn(NodeBundle {
                            background_color: BackgroundColor(Color::BLUE),
                            style: TABS_COMPARTMENT_STYLE,
                            ..default()
                        })
                        .insert(LevelEditorTabs)
                        .with_children(|parent| {
                            parent
                                .spawn(ButtonBundle::default())
                                .insert(NodeBundle {
                                    background_color: BackgroundColor(Color::MIDNIGHT_BLUE),
                                    visibility: Visibility::Visible,
                                    style: TABS_STYLE,
                                    ..default()
                                })
                                .insert(LevelEditorTab(1));
                            for i in 2..=MAX_MAPS {
                                parent
                                    .spawn(ButtonBundle::default())
                                    .insert(NodeBundle {
                                        background_color: BackgroundColor(Color::MIDNIGHT_BLUE),
                                        visibility: Visibility::Hidden,
                                        style: TABS_STYLE,
                                        ..default()
                                    })
                                    .insert(LevelEditorTab(i));
                            }
                            parent
                                .spawn(ButtonBundle::default())
                                .insert(ImageBundle {
                                    image: UiImage {
                                        texture: plus_image,
                                        ..default()
                                    },
                                    style: PLUS_STYLE,
                                    ..default()
                                })
                                .insert(LevelEditorTabPlus);
                        });
                });
            parent
                .spawn(NodeBundle {
                    background_color: BackgroundColor(Color::GREEN),
                    visibility: Visibility::Visible,
                    style: RIGHT_COMPARTMENT_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            background_color: BackgroundColor(Color::DARK_GREEN),
                            visibility: Visibility::Visible,
                            style: OBJECTS_COMPARTMENT_STYLE,
                            ..default()
                        })
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
                    parent
                        .spawn(NodeBundle {
                            background_color: BackgroundColor(Color::GREEN),
                            visibility: Visibility::Visible,
                            style: FLOORS_COMPARTMENT_STYLE,
                            ..default()
                        })
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
                                spawn_small_button(
                                    parent,
                                    turtle_image.clone(),
                                    GameEntity::Object(GameObject::Turtle { color }),
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
                });
        });
}
