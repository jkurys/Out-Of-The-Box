use bevy::prelude::*;

use crate::{
    game::{display::DisplayPlugin, movement::MovementPlugin, GamePlugin},
    init_images,
    menu::MenusPlugin,
    resources::{CurrentLevel, CurrentSprite},
    spawn_camera,
    state::{DisplayState, MoveState},
    update_images,
};

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentLevel {
            level_number: 1,
            level_amount: 0,
            level_map_string: "".to_string(),
            is_in_level: false,
        })
        .insert_resource(CurrentSprite(0));

        app.add_plugins(DefaultPlugins)
        .add_plugins(MenusPlugin)
        .add_plugins(GamePlugin);

        app.add_plugins(DisplayPlugin)
        .add_plugins(MovementPlugin)
        .add_systems(Update, transition_to_in_menu.run_if(in_state(DisplayState::Setup)))
        .add_systems(OnEnter(DisplayState::Setup), spawn_camera)
        .add_systems(OnEnter(DisplayState::Setup), init_images)
        .add_systems(Update, update_images);
        app.init_state::<DisplayState>()
        .init_state::<MoveState>();
        // .add_systems(Update, spritemap_fix);
    }
}


fn transition_to_in_menu(mut app_state: ResMut<NextState<DisplayState>>) {
    app_state.set(DisplayState::MainMenu);
}
