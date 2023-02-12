use bevy::prelude::*;
use consts::*;
use std::fs::File;
use std::io::Read;
use game::display::DisplayPlugin;
use game::movement::MovementPlugin;
use game::GamePlugin;
use menu::MenusPlugin;
use resources::*;
use state::{CurrentMap, DisplayState, GameState};
mod consts;
mod exit;
mod game;
mod labels;
mod menu;
mod resources;
mod state;
mod utils;

fn main() {
    App::new()
        .insert_resource(CurrentLevel {
            level_number: 1,
            level_map_str: MAP_NAMES[0],
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(MenusPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(DisplayPlugin)
        .add_plugin(MovementPlugin)
        .add_state(DisplayState::MainMenu)
        .add_state(GameState(None))
        .add_state(CurrentMap(None))
        .add_startup_system(spawn_camera)
        .add_system(update_images)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update_images(asset_server: ResMut<AssetServer>, mut images: ResMut<Images>) {
        let mut file = File::open("assets/saves/save.txt").unwrap();
        let mut buf = [0 as u8; 1];
        file.read(&mut buf).unwrap();
        images.player_image = asset_server.load(PLAYER_TEXTURES[buf[0] as usize]);
}