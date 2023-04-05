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

use super::{
    resources::LevelEditorBoard,
    utils::{spawn_small_button, spawn_small_image},
};

#[derive(Component)]
pub struct LevelEditorItem;

#[derive(Component)]
pub struct LevelEditorTabs;

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
                            parent.spawn(NodeBundle {
                                background_color: BackgroundColor(Color::MIDNIGHT_BLUE),
                                style: Style {
                                    size: Size {
                                        height: Val::Percent(100.),
                                        width: Val::Percent(10.),
                                    },
                                    ..default()
                                },
                                ..default()
                            });
                            parent
                                .spawn(ButtonBundle::default())
                                .insert(ImageBundle {
                                    image: UiImage(images.tile_image.clone()), // tu będzie plusik
                                    style: Style {
                                        size: Size {
                                            width: Val::Px(20.),
                                            height: Val::Px(20.),
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

pub fn handle_level_editor_click(
    mut changable_query: Query<
        (&LevelEditorChangable, &Interaction, &mut UiImage),
        With<LevelEditorChangable>,
    >,
    clickable_query: Query<(&Interaction, &UiImage, &GameEntity), Without<LevelEditorChangable>>,
    mut board: ResMut<LevelEditorBoard>,
    mut current_object: Local<GameEntity>,
    mut last_added_player: Local<(Option<Position>, bool)>,
    images: Res<Images>,
) {
    for (changable, interaction, mut image) in changable_query.iter_mut() {
        let position = changable.0;
        if *interaction == Interaction::Clicked {
            if let (Some(prev_position), false) = *last_added_player {
                if let GameEntity::Object(GameObject::Player) = *current_object {
                    if position != prev_position {
                        *last_added_player = (Some(prev_position), true);
                    }
                }
            } else if let GameEntity::Object(GameObject::Player) = *current_object {
                *last_added_player = (Some(position), false);
            }
            *image = board.image.clone();
            board.objects.insert(position, *current_object);
        }
        if let (Some(player_position), true) = *last_added_player {
            if player_position == position {
                *image = UiImage(images.tile_image.clone());
                board.objects.remove(&position);
            }
        }
    }
    for (interaction, image, object_or_floor) in clickable_query.iter() {
        if *interaction == Interaction::Clicked {
            board.image = image.clone();
            *current_object = *object_or_floor;
        }
    }
}

// translates the position to index in a 1d array with '\n' at the end of each line
fn position_to_index(pos: Position, width: u32, height: u32) -> usize {
    let x = pos.x + (width / 2) as i32;
    let y = pos.y + (height / 2) as i32;
    x as usize
        + y as usize * (width + 1) as usize
}

pub fn save_board_to_file(board: Res<LevelEditorBoard>) {
    let mut height_string: Vec<char> = board.height.to_string().chars().collect();
    let mut width_string: Vec<char> = board.width.to_string().chars().collect();
    let mut map_prelude = vec!['1', '\n'];
    map_prelude.append(&mut height_string);
    map_prelude.append(&mut vec![' ']);
    map_prelude.append(&mut width_string);
    map_prelude.append(&mut vec!['\n']);
    let mut buf = vec![' '; (board.width + 1) as usize * board.height as usize];
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
    let mut file = File::create("assets/saves/map.txt").unwrap();
    map_prelude.append(&mut buf);
    let buf = map_prelude.iter().map(|c| *c as u8).collect::<Vec<_>>();
    file.write_all(&buf[..]).unwrap();
}
