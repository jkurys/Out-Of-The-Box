use bevy::prelude::*;

use crate::{consts::{HOVERED_PLUS_TEXTURE, PLUS_TEXTURE}, state::DisplayState, resources::Board};

use super::editor::{LevelEditorTabPlus, LevelEditorTab, LevelEditorChangable, GameEntity};

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
            *image = UiImage{ texture: hovered_plus_image, ..default() };
            if *is_clicked {
                *is_clicked = false;
                *tabs_amount += 1;
                for (mut style, mut visible) in tab_query.iter_mut() {
                    if *visible != Visibility::Visible {
                        style.display = Display::Flex;
                        *visible = Visibility::Visible;
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
            *image = UiImage{ texture: plus_image, ..default() };
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
    mut boards: ResMut<Board>,
    mut app_state: ResMut<NextState<DisplayState>>
) {
    for (tab_num, interaction, mut color) in tab_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                boards.set_current_map(tab_num.0 - 1);
                app_state.set(DisplayState::LevelEditorInput);
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
            }
        }
    }
    for (interaction, new_image, object_or_floor) in clickable_query.iter() {
        if *interaction == Interaction::Clicked {
            image.0 = new_image.clone();
            image.1 = true;
            *current_object = *object_or_floor;
        }
    }
}