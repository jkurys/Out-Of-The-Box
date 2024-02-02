use bevy::prelude::*;
use std::fs::File;
use std::io::Write;

use crate::consts::PLAYER_TEXTURE_SAVE;
use crate::state::DisplayState;

#[derive(Component)]
pub struct SpriteSelectItem;

#[derive(Component)]
pub enum SpriteSelectItemType {
    Select(u8),
    Back,
}

pub fn handle_sprite_click(
    mut app_state: ResMut<NextState<DisplayState>>,
    mut query: Query<
        (
            &mut Interaction,
            &mut BackgroundColor,
            &mut SpriteSelectItemType,
        ),
        With<SpriteSelectItemType>,
    >,
) {
    query.for_each_mut(
        |(interaction, mut color, item)| match interaction.as_ref() {
            Interaction::Pressed => match item.as_ref() {
                SpriteSelectItemType::Back => {
                    app_state.set(DisplayState::MainMenu);
                }
                SpriteSelectItemType::Select(sprite_no) => {
                    let bytes = [*sprite_no];
                    let mut file = File::create(PLAYER_TEXTURE_SAVE).unwrap();
                    file.write_all(&bytes).unwrap();
                    app_state.set(DisplayState::MainMenu);
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
