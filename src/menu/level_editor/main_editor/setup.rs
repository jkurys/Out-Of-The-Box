use bevy::prelude::*;
use bevy::color::palettes::css::{BEIGE, DARK_GREEN, GREEN};

use crate::game::game_objects::PowerUpType;
use crate::{
    board::Board,
    components::GameEntity,
    consts::{
        BOX_TEXTURE, HIDDEN_WALL_TEXTURES, PLAYER_TEXTURES, 
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

// use super::{LevelEditorTab, LevelEditorTabPlus};

pub fn set_board_size(board_size: Res<BoardSize>, mut boards: ResMut<Board>) {
    boards.set_map_size(*board_size);
    boards.init_objs();
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
    // let plus_image = asset_server.load(PLUS_TEXTURE);
    let button_images = [asset_server.load(BUTTON_TEXTURES[0]), asset_server.load(BUTTON_TEXTURES[1]), asset_server.load(BUTTON_TEXTURES[2])];
    commands.spawn(NodeBundle {
        visibility: Visibility::Hidden,
        background_color: BackgroundColor(Color::Srgba(BEIGE)),
        // background_color: BackgroundColor(Color::BEIGE),
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        ..default()
    }).insert(LevelEditorItem)
        .with_children(|parent| {

        parent.spawn(NodeBundle {
            background_color: BackgroundColor(Color::Srgba(BEIGE)),
            // background_color: BackgroundColor(Color::BEIGE),
            visibility: Visibility::Visible,
            style: Style {
                width: Val::Percent(10.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                align_self: AlignSelf::End,
                ..default()
            },
            ..default()

        })
        .insert(LevelEditorItem)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    background_color: BackgroundColor(Color::Srgba(DARK_GREEN)),
                    // background_color: BackgroundColor(Color::DARK_GREEN),
                    visibility: Visibility::Visible,
                    style: Style {
                        width: Val::Percent(100.0),
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
                            GameEntity::Object(GameObject::HidingWall { color, hidden_toggle: false, hidden_by_def: false }),
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
                        GameEntity::Object(GameObject::Player { powerup: None, direction: Direction::South }),
                    );
                    spawn_small_button(
                        parent,
                        images.powerup_images.clone().unwrap().0,
                        GameEntity::Object(GameObject::PowerUp { powerup_type: PowerUpType::Rocket }),
                    );
                });

            parent
                .spawn(NodeBundle {
                    background_color: BackgroundColor(Color::Srgba(GREEN)),
                    // background_color: BackgroundColor(Color::GREEN),
                    visibility: Visibility::Visible,
                    style: Style {
                        width: Val::Percent(100.0),
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
                            GameEntity::Object(GameObject::HidingWall {
                                color,
                                hidden_toggle: true,
                                hidden_by_def: true,
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
                }
            );


        });
        

    });
}
