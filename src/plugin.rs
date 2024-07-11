use bevy::prelude::*;

use crate::{
    game::{display::DisplayPlugin, movement::MovementPlugin, GamePlugin},
    init_images,
    menu::MenusPlugin,
    resources::{CurrentLevel, CurrentSprite},
    spawn_camera, spritemap_fix,
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
        .insert_resource(CurrentSprite(0))
        .add_state::<DisplayState>()
        .add_state::<MoveState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(MenusPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(DisplayPlugin)
        .add_plugins(MovementPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, init_images)
        .add_systems(Update, update_images)
        .add_systems(Update, spritemap_fix);
    }
}
