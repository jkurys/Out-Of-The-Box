use bevy::prelude::*;

use crate::consts::{HOVERED_PLUS_TEXTURE, PLUS_TEXTURE};

use super::{LevelEditorTab, LevelEditorTabPlus};

pub fn handle_plus_click(
    mut plus_query: Query<
        (&mut Style, &Interaction, &mut UiImage),
        (With<LevelEditorTabPlus>, Without<LevelEditorTab>),
    >,
    mut tab_query: Query<
        (&mut Style, &mut Visibility),
        (With<LevelEditorTab>, Without<LevelEditorTabPlus>),
    >,
    asset_server: Res<AssetServer>,
    mut tabs_amount: Local<u32>,
    mut is_clicked: Local<bool>,
) {
    let hovered_plus_image = asset_server.load(HOVERED_PLUS_TEXTURE);
    let plus_image = asset_server.load(PLUS_TEXTURE);
    let (mut plus_style, interaction, mut image) = plus_query.single_mut();
    match interaction {
        Interaction::Hovered => {
            *image = UiImage {
                texture: hovered_plus_image,
                ..default()
            };
            if *is_clicked {
                *is_clicked = false;
                *tabs_amount += 1;
                for (mut style, mut visible) in tab_query.iter_mut() {
                    if *visible == Visibility::Hidden {
                        style.display = Display::Flex;
                        *visible = Visibility::Visible;
                        if *tabs_amount >= 9 {
                            plus_style.display = Display::None;
                        }
                        break;
                    }
                }
            }
        }
        Interaction::None => {
            *is_clicked = false;
            *image = UiImage {
                texture: plus_image,
                ..default()
            };
        }
        Interaction::Pressed => {
            *is_clicked = true;
        }
    }
}
