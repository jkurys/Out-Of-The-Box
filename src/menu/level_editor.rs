use bevy::prelude::*;

use std::fs::File;
use std::io::Write;

use crate::{
    consts::{MAIN_MENU_FONT, MAX_HEIGHT, MAX_WIDTH},
    game::game_objects::{Floor, GameObject, Position},
    resources::Images,
    state::DisplayState,
    utils::offset_coordinate,
};

use super::resources::LevelEditorBoard;

#[derive(Component)]
pub struct LevelEditorItem;

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

pub fn setup_level_editor(asset_server: Res<AssetServer>, mut commands: Commands) {
    let menu_font = asset_server.load(MAIN_MENU_FONT);
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::BLACK),
            visibility: Visibility { is_visible: true },
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        })
        .insert(LevelEditorItem)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Please provide the level width",
                TextStyle {
                    font: menu_font.clone(),
                    font_size: 50.,
                    color: Color::WHITE,
                },
            ));
        });
}

pub fn handle_level_editor_input(
    mut char_reader: EventReader<ReceivedCharacter>,
    mut input: ResMut<Input<KeyCode>>,
    mut width: Local<u32>,
    mut height: Local<u32>,
    mut is_width_provided: Local<bool>,
    mut app_state: ResMut<State<DisplayState>>,
) {
    for ev in char_reader.iter() {
        if ev.char.is_ascii_digit() && !*is_width_provided {
            *width = *width * 10 + ev.char.to_digit(10).unwrap();
            if *width > MAX_WIDTH {
                *width = 0;
            }
        } else if ev.char.is_ascii_digit() {
            *height = *height * 10 + ev.char.to_digit(10).unwrap();
            if *height > MAX_HEIGHT {
                *height = 0;
            }
        }
    }
    if input.just_pressed(KeyCode::Return) && !*is_width_provided {
        *is_width_provided = true;
        input.reset(KeyCode::Return);
    }
    if input.just_pressed(KeyCode::Return) && *is_width_provided {
        *is_width_provided = false;
        app_state
            .set(DisplayState::LevelEditorBoard(*width, *height))
            .expect("Could not get display state input");
        *height = 0;
        *width = 0;
    }
}

pub fn setup_level_editor_board(
    mut commands: Commands,
    images: Res<Images>,
    state: Res<State<DisplayState>>,
    mut board: ResMut<LevelEditorBoard>,
) {
    let (width, height) = if let DisplayState::LevelEditorBoard(width, height) = state.current() {
        (*width, *height)
    } else {
        (1, 1)
    };
    board.width = width;
    board.height = height;
    let bottom_border = offset_coordinate(0, height as i32);
    let top_border = offset_coordinate(height as i32 - 1, height as i32);
    let left_border = offset_coordinate(0, width as i32);
    let right_border = offset_coordinate(width as i32 - 1, width as i32);
    // let menu_font = asset_server.load(MAIN_MENU_FONT);
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
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .insert(LevelEditorItem)
                // board
                .with_children(|parent| {
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
                                parent.spawn(ImageBundle {
                                    image: UiImage(images.wall_image.clone()),
                                    style: Style {
                                        size: Size {
                                            height: Val::Px(50.),
                                            width: Val::Px(50.),
                                        },
                                        ..default()
                                    },
                                    ..default()
                                });
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
                                parent.spawn(ImageBundle {
                                    image: UiImage(images.wall_image.clone()),
                                    style: Style {
                                        size: Size {
                                            height: Val::Px(50.),
                                            width: Val::Px(50.),
                                        },
                                        ..default()
                                    },
                                    ..default()
                                });
                                // inside tiles
                                for y in bottom_border..(top_border + 1) {
                                    parent
                                        .spawn(ButtonBundle::default())
                                        .insert(ImageBundle {
                                            image: UiImage(images.tile_image.clone()),
                                            style: Style {
                                                size: Size {
                                                    height: Val::Px(50.),
                                                    width: Val::Px(50.),
                                                },
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .insert(LevelEditorChangable(Position { x, y, map: 0 }));
                                }
                                // bottom wall
                                parent.spawn(ImageBundle {
                                    image: UiImage(images.wall_image.clone()),
                                    style: Style {
                                        size: Size {
                                            height: Val::Px(50.),
                                            width: Val::Px(50.),
                                        },
                                        ..default()
                                    },
                                    ..default()
                                });
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
                                parent.spawn(ImageBundle {
                                    image: UiImage(images.wall_image.clone()),
                                    style: Style {
                                        size: Size {
                                            height: Val::Px(50.),
                                            width: Val::Px(50.),
                                        },
                                        ..default()
                                    },
                                    ..default()
                                });
                            }
                        });
                });
            parent
                .spawn(NodeBundle {
                    background_color: BackgroundColor(Color::GREEN),
                    visibility: Visibility { is_visible: true },
                    style: Style {
                        size: Size {
                            width: Val::Percent(30.0),
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
                    parent
                        .spawn(ButtonBundle::default())
                        .insert(ImageBundle {
                            image: UiImage(images.box_image.clone()),
                            style: Style {
                                size: Size {
                                    height: Val::Px(50.),
                                    width: Val::Px(50.),
                                },
                                ..default()
                            },
                            ..default()
                        })
                        .insert(GameEntity::Object(GameObject::Box));
                    parent
                        .spawn(ButtonBundle::default())
                        .insert(ImageBundle {
                            image: UiImage(images.goal_image.clone()),
                            style: Style {
                                size: Size {
                                    height: Val::Px(50.),
                                    width: Val::Px(50.),
                                },
                                ..default()
                            },
                            ..default()
                        })
                        .insert(GameEntity::Floor(Floor::Goal));
                    parent
                        .spawn(ButtonBundle::default())
                        .insert(ImageBundle {
                            image: UiImage(images.ice_image.clone()),
                            style: Style {
                                size: Size {
                                    height: Val::Px(50.),
                                    width: Val::Px(50.),
                                },
                                ..default()
                            },
                            ..default()
                        })
                        .insert(GameEntity::Floor(Floor::Ice));
                });
        });
}

pub fn handle_level_editor_click(
    mut changable_query: Query<
        (&LevelEditorChangable, &Interaction, &mut UiImage),
        With<LevelEditorChangable>,
    >,
    clickable_query: Query<(&Interaction, &UiImage, &GameEntity), Without<LevelEditorChangable>>,
    mut board: ResMut<LevelEditorBoard>,
    mut current_object: Local<GameEntity>,
) {
    for (changable, interaction, mut image) in changable_query.iter_mut() {
        let position = changable.0;
        if *interaction == Interaction::Clicked {
            *image = board.image.clone();
            board.objects.insert(position, *current_object);
        }
    }
    for (interaction, image, object_or_floor) in clickable_query.iter() {
        if *interaction == Interaction::Clicked {
            board.image = image.clone();
            *current_object = *object_or_floor;
        }
    }
}

pub fn position_to_index(pos: Position, width: u32, height: u32) -> usize {
    let x = pos.x + (width / 2) as i32;
    let y = pos.y + (height / 2) as i32;
    x as usize
        + y as usize * (width + 1) as usize
        + pos.map as usize * (width + 1) as usize * height as usize
}

pub fn save_board_to_file(board: Res<LevelEditorBoard>) {
    let mut buf = vec!['.'; (board.width + 1) as usize * board.height as usize];
    for i in 0..board.height {
        buf[(board.width + i * (board.width + 1)) as usize] = '\n';
    }
    for (position, object) in board.objects.iter() {
        let index = position_to_index(*position, board.width, board.height);
        buf[index] = match *object {
            GameEntity::Object(object) => match object {
                GameObject::Box => 'b',
                GameObject::Wall => 'w',
                GameObject::HidingWall => 'H',
                GameObject::Empty => ' ',
                GameObject::Player => 'p',
            },
            GameEntity::Floor(floor) => match floor {
                Floor::HiddenWall {
                    hidden_by_default: _,
                } => 'h',
                Floor::Tile => ' ',
                Floor::Ice => 'i',
                Floor::Goal => 'g',
                Floor::Warp(num) => char::from_digit(num as u32, 10).unwrap(),
                Floor::Button => 'u',
            },
        }
    }
    // let s: String = buf.iter().collect();
    let mut file = File::create("assets/saves/map.txt").unwrap();
    let buf = buf.iter().map(|c| *c as u8).collect::<Vec<_>>();
    file.write_all(&buf[..]).unwrap();
}
