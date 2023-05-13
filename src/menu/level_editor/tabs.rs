use bevy::prelude::*;

use crate::{consts::{HOVERED_PLUS_TEXTURE, PLUS_TEXTURE}, state::DisplayState, game::game_objects::{Position, GameObject}, resources::Images};

use super::{editor::{LevelEditorTabPlus, LevelEditorTab, LevelEditorChangable, GameEntity}, resources::LevelEditorBoard};

pub fn handle_plus_click(
    mut plus_query: Query<
        (&mut Style, &Interaction, &mut UiImage),
        (With<LevelEditorTabPlus>, Without<LevelEditorTab>),
    >,
    mut tab_query: Query<(&mut Style, &mut Visibility), (With<LevelEditorTab>, Without<LevelEditorTabPlus>)>,
    asset_server: Res<AssetServer>,
    mut tabs_amount: Local<u32>,
    mut is_clicked: Local<bool>,
) {
    let hovered_plus_image = asset_server.load(HOVERED_PLUS_TEXTURE);
    let plus_image = asset_server.load(PLUS_TEXTURE);
    let (mut plus_style, interaction, mut image) = plus_query.single_mut();
    match interaction {
        Interaction::Hovered => {
            *image = UiImage(hovered_plus_image);
            if *is_clicked {
                *is_clicked = false;
                *tabs_amount += 1;
                for (mut style, mut visible) in tab_query.iter_mut() {
                    if !visible.is_visible {
                        style.display = Display::Flex;
                        visible.is_visible = true;
                        if *tabs_amount >= 9 {
                            plus_style.display = Display::None;
                        }
                        break;
                    }
                }
            }
        },
        Interaction::None => {
            *is_clicked = false;
            *image = UiImage(plus_image);
        }
        Interaction::Clicked => {
            *is_clicked = true;
        }
    }
}

pub fn handle_tab_click(
    mut tab_query: Query<
        (&LevelEditorTab, &Interaction, &mut BackgroundColor),
        With<LevelEditorTab>,
    >,
    mut boards: ResMut<LevelEditorBoard>,
    mut app_state: ResMut<State<DisplayState>>
) {
    for (tab_num, interaction, mut color) in tab_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                boards.curr_map = tab_num.0 - 1;
                boards.init_map_n(tab_num.0 - 1);
                app_state.set(DisplayState::LevelEditorInput).expect("Could not go back to input");
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::rgb(0.25, 0.25, 1.));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::MIDNIGHT_BLUE);
            }
        }
    }
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
            board.insert_object(position, *current_object);
        }
        if let (Some(player_position), true) = *last_added_player {
            if player_position == position {
                *image = UiImage(images.tile_image.clone());
                board.remove_object(position);
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