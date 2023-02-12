use bevy::prelude::*;
use std::fs::File;
use std::io::Write;
use crate::{consts::MAIN_MENU_FONT, state::DisplayState};
use super::spawn_button;
#[derive(Component)]
pub struct SpriteSelectItem;

#[derive(Component)]
pub enum SpriteSelectItemType {
    Select(u8),
    Back,
}

pub fn setup_sprite_select(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        .insert(SpriteSelectItem)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Select Sprite",
                    TextStyle {
                        font: menu_font.clone(),
                        font_size: 50.,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }),
            );
            spawn_button(parent, SpriteSelectItemType::Back, menu_font.clone(), "back");
            spawn_button(parent, SpriteSelectItemType::Select(0), menu_font.clone(), "select 0");
            spawn_button(parent, SpriteSelectItemType::Select(1), menu_font.clone(), "select 1");
            spawn_button(parent, SpriteSelectItemType::Select(2), menu_font.clone(), "select 2");
        });
}

pub fn handle_sprite_click(
    mut app_state: ResMut<State<DisplayState>>,
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
            Interaction::Clicked => match item.as_ref() {
                SpriteSelectItemType::Back => {
                    app_state.pop().expect("Going back to main menu failed");
                }
                SpriteSelectItemType::Select(sprite_no) => {
                    let bytes = [*sprite_no];
                    let mut file = File::create("assets/saves/save.txt").unwrap();
                    file.write_all(&bytes).unwrap();
                    app_state.pop().expect("Going back to main menu failed");
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