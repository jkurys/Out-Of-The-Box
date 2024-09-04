use bevy::{app::AppExit, prelude::*};
use bevy::color::palettes::css::{GRAY, WHITE};

use crate::state::DisplayState;

use super::MenuItemType;

pub fn handle_menu_click(
    mut app_state: ResMut<NextState<DisplayState>>,
    mut query: Query<
        (&mut Interaction, &mut BackgroundColor, &mut MenuItemType),
        With<MenuItemType>,
    >,
    mut app_exit: EventWriter<AppExit>,
) {
    query.iter_mut().for_each(
        |(interaction, mut color, item)| match interaction.as_ref() {
            Interaction::Pressed => match item.as_ref() {
                MenuItemType::LevelSelect => {
                    app_state.set(DisplayState::LevelSelect);
                }
                MenuItemType::SpriteSelect => {
                    app_state.set(DisplayState::SpriteSelect);
                }
                MenuItemType::Exit => {
                    app_exit.send(AppExit::Success);
                    // app_exit.send(AppExit);
                }
                MenuItemType::LevelEditor => {
                    app_state.set(DisplayState::LevelEditorSelect);
                }
            },
            Interaction::Hovered => {
                *color = BackgroundColor(Color::Srgba(GRAY));
                // *color = BackgroundColor(Color::GRAY);
            }
            Interaction::None => {
                *color = BackgroundColor(Color::Srgba(WHITE));
                // *color = BackgroundColor(Color::WHITE);
            }
        },
    )
}
