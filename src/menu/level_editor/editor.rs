use bevy::prelude::*;

use crate::{
    consts::*,
    game::game_objects::{Floor, GameObject, Position},
    resources::Images,
    state::DisplayState,
    utils::offset_coordinate,
};

use super::{
    resources::LevelEditorBoard,
    utils::{spawn_small_button, spawn_small_image},
};

#[derive(Component)]
pub struct LevelEditorItem;

#[derive(Component)]
pub struct LevelEditorInputNumber;

#[derive(Component)]
pub struct Invisibility(bool);

#[derive(Component)]
pub struct LevelEditorStartingPrompt;

#[derive(Component)]
pub struct LevelEditorTabs;

#[derive(Component)]
pub struct LevelEditorTab(pub usize);

#[derive(Component)]
pub struct LevelEditorTabPlus;

#[derive(Component, Clone, Copy, Debug)]
pub enum GameEntity {
    Object(GameObject),
    Floor(Floor),
}

impl Default for GameEntity {
    fn default() -> Self {
        GameEntity::Object(GameObject::Wall)
    }
}

#[derive(Component)]
pub struct LevelEditorChangable(pub Position);

pub fn setup_level_editor_board(
    mut commands: Commands,
    images: Res<Images>,
    state: Res<State<DisplayState>>,
    mut boards: ResMut<LevelEditorBoard>,
    asset_server: Res<AssetServer>,
) {
    let (width, height) = if let DisplayState::LevelEditorBoard(width, height) = state.current() {
        (*width, *height)
    } else {
        (1, 1)
    };
    boards.set_size(width, height);
    let plus_image = asset_server.load(PLUS_TEXTURE);
    let bottom_border = offset_coordinate(0, height as i32);
    let top_border = offset_coordinate(height as i32 - 1, height as i32);
    let left_border = offset_coordinate(0, width as i32);
    let right_border = offset_coordinate(width as i32 - 1, width as i32);
    commands
        // main separation between 2 compartments
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::BLACK),
            visibility: Visibility { is_visible: true },
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        })
        .insert(LevelEditorItem)
        .with_children(|parent| {
            //left compartment, with board
            parent
                .spawn(NodeBundle {
                    background_color: BackgroundColor(Color::GRAY),
                    visibility: Visibility { is_visible: true },
                    style: Style {
                        size: Size {
                            width: Val::Percent(70.0),
                            height: Val::Percent(100.0),
                        },
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                // two compartments: tabs and board
                .with_children(|parent| {
                    
                    // board, component holding all columns
                    parent
                        .spawn(NodeBundle {
                            background_color: BackgroundColor(Color::GRAY),
                            visibility: Visibility { is_visible: true },
                            style: Style {
                                size: Size {
                                    width: Val::Percent(70.0),
                                    height: Val::Percent(100.0),
                                },
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // component for a left-most column
                            parent
                                .spawn(NodeBundle {
                                    background_color: BackgroundColor(Color::GRAY),
                                    visibility: Visibility { is_visible: true },
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    for _ in 0..height + 2 {
                                        //left-most column, all walls
                                        spawn_small_image(parent, images.wall_image.clone());
                                    }
                                });
                            for x in left_border..(right_border + 1) {
                                // middle columns
                                parent
                                    .spawn(NodeBundle {
                                        background_color: BackgroundColor(Color::GRAY),
                                        visibility: Visibility { is_visible: true },
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        //top wall
                                        spawn_small_image(parent, images.wall_image.clone());
                                        // inside tiles
                                        for y in bottom_border..(top_border + 1) {
                                            spawn_small_button(
                                                parent,
                                                images.tile_image.clone(),
                                                LevelEditorChangable(Position { x, y }),
                                            );
                                        }
                                        // bottom wall
                                        spawn_small_image(parent, images.wall_image.clone());
                                    });
                            }
                            // right-most column, all walls
                            parent
                                .spawn(NodeBundle {
                                    background_color: BackgroundColor(Color::GRAY),
                                    visibility: Visibility { is_visible: true },
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    for _ in 0..height + 2 {
                                        spawn_small_image(parent, images.wall_image.clone());
                                    }
                                });
                        });
                    // map tabs
                    parent
                        .spawn(NodeBundle {
                            background_color: BackgroundColor(Color::BLUE),
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::FlexStart,
                                size: Size {
                                    width: Val::Percent(100.),
                                    height: Val::Percent(5.),
                                },
                                ..default()
                            },
                            ..default()
                        })
                        .insert(LevelEditorTabs)
                        // first tab
                        .with_children(|parent| {
                            parent.spawn(ButtonBundle::default())
                                .insert(NodeBundle {
                                    background_color: BackgroundColor(Color::MIDNIGHT_BLUE),
                                    visibility: Visibility { is_visible: true },
                                    style: Style {
                                        size: Size {
                                            height: Val::Percent(100.),
                                            width: Val::Percent(10.),
                                        },
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert(LevelEditorTab(1));
                                // .insert(Invisibility(false));
                            for i in 2..11 {
                                parent.spawn(ButtonBundle::default())
                                    .insert(NodeBundle {
                                        background_color: BackgroundColor(Color::MIDNIGHT_BLUE),
                                        visibility: Visibility { is_visible: false },
                                        style: Style {
                                            size: Size {
                                                height: Val::Percent(100.),
                                                width: Val::Percent(10.),
                                            },
                                            display: Display::None,
                                            ..default()
                                        },
                                        ..default()
                                    })
                                .insert(LevelEditorTab(i));
                                // .insert(Invisibility(true));
                            }
                            parent
                                .spawn(ButtonBundle::default())
                                .insert(ImageBundle {
                                    image: UiImage(plus_image),
                                    style: Style {
                                        size: Size {
                                            width: Val::Px(20.),
                                            height: Val::Px(20.),
                                        },
                                        margin: UiRect {
                                            left: Val::Px(10.),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert(LevelEditorTabPlus);
                        });
                });
            // right section of the editor
            parent
                .spawn(NodeBundle {
                    background_color: BackgroundColor(Color::GREEN),
                    visibility: Visibility { is_visible: true },
                    style: Style {
                        size: Size {
                            width: Val::Percent(30.0),
                            height: Val::Percent(100.0),
                        },
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // objects
                    parent
                        .spawn(NodeBundle {
                            background_color: BackgroundColor(Color::DARK_GREEN),
                            visibility: Visibility { is_visible: true },
                            style: Style {
                                size: Size {
                                    width: Val::Percent(50.0),
                                    height: Val::Percent(100.0),
                                },
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceEvenly,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            spawn_small_button(
                                parent,
                                images.box_image.clone(),
                                GameEntity::Object(GameObject::Box),
                            );
                            spawn_small_button(
                                parent,
                                images.shown_hidden_wall_image.clone(),
                                GameEntity::Object(GameObject::HidingWall),
                            );
                            spawn_small_button(
                                parent,
                                images.wall_image.clone(),
                                GameEntity::Object(GameObject::Wall),
                            );
                            spawn_small_button(
                                parent,
                                images.player_image.clone(),
                                GameEntity::Object(GameObject::Player),
                            );
                        });
                    // floors
                    parent
                        .spawn(NodeBundle {
                            background_color: BackgroundColor(Color::GREEN),
                            visibility: Visibility { is_visible: true },
                            style: Style {
                                size: Size {
                                    width: Val::Percent(50.0),
                                    height: Val::Percent(100.0),
                                },
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceEvenly,
                                ..default()
                            },
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
                            spawn_small_button(
                                parent,
                                images.button_image.clone(),
                                GameEntity::Floor(Floor::Button),
                            );
                            spawn_small_button(
                                parent,
                                images.hidden_wall_image.clone(),
                                GameEntity::Floor(Floor::HiddenWall {
                                    hidden_by_default: true,
                                }),
                            );
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
