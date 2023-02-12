use bevy::{app::AppExit, prelude::*};

use super::spawn_button;
use crate::{consts::MAIN_MENU_FONT, state::DisplayState};

#[derive(Component)]
pub struct MainMenuItem;

#[derive(Component)]
pub enum MenuItemType {
    LevelSelect,
    Exit,
    SpriteSelect,
}

pub fn handle_menu_click(
    mut app_state: ResMut<State<DisplayState>>,
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
                    app_state
                        .push(DisplayState::LevelSelect)
                        .expect("Could not load level select");
                }
                MenuItemType::SpriteSelect => {
                    app_state.push(DisplayState::SpriteSelect).expect("Could not load sprite select");
                }
                MenuItemType::Exit => {
                    app_exit.send(AppExit);
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

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        .insert(MainMenuItem)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Main Menu",
                    TextStyle {
                        font_size: 50.0,
                        color: Color::WHITE,
                        font: menu_font.clone(),
                    },
                )
                .with_text_alignment(TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }),
            );
            spawn_button(
                parent,
                MenuItemType::LevelSelect,
                menu_font.clone(),
                "Level Select",
            );
            spawn_button(parent, MenuItemType::SpriteSelect, menu_font.clone(), "Sprite Select");
            spawn_button(parent, MenuItemType::Exit, menu_font.clone(), "Exit");
        });
}
