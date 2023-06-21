use bevy::{app::AppExit, prelude::*};

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
    query.for_each_mut(
        |(interaction, mut color, item)| match interaction.as_ref() {
            Interaction::Clicked => match item.as_ref() {
                MenuItemType::LevelSelect => {
                    app_state.set(DisplayState::LevelSelect);
                }
                MenuItemType::SpriteSelect => {
                    app_state.set(DisplayState::SpriteSelect);
                }
                MenuItemType::Exit => {
                    app_exit.send(AppExit);
                }
                MenuItemType::LevelEditor => {
                    app_state.set(DisplayState::LevelEditorSelect);
                }
            },
            Interaction::Hovered => {
                *color = BackgroundColor(Color::GRAY);
            }
            Interaction::None => {
                *color = BackgroundColor(Color::WHITE);
            }
        },
    )
}
