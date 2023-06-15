use bevy::prelude::*;
use consts::*;
use game::display::DisplayPlugin;
use game::movement::MovementPlugin;
use game::GamePlugin;
use menu::MenusPlugin;
use resources::*;
use state::{DisplayState, MoveState};
use std::fs::File;
use std::io::Read;
mod consts;
mod exit;
mod game;
mod board;
mod menu;
mod resources;
mod state;
mod utils;
mod components;

fn main() {
    App::new()
        .insert_resource(CurrentLevel {
            level_number: 1,
            level_amount: 0,
            level_map_string: "".to_string(),
        })
        .add_state::<DisplayState>()
        .add_state::<MoveState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(MenusPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(DisplayPlugin)
        .add_plugin(MovementPlugin)
        .add_startup_system(spawn_camera)
        .add_system(update_images)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update_images(asset_server: ResMut<AssetServer>, mut images: ResMut<Images>) {
    let mut file = File::open(PLAYER_TEXTURE_SAVE).unwrap();
    let mut buf = [0_u8; 1];
    file.read_exact(&mut buf).unwrap();
    images.player_images = [asset_server.load(LOWER_PLAYER_TEXTURES[buf[0] as usize]), asset_server.load(PLAYER_TEXTURES[buf[0] as usize])];
}
