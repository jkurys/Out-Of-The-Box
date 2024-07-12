use bevy::prelude::*;
use consts::*;
use game::display::DisplayPlugin;
use game::movement::MovementPlugin;
use game::GamePlugin;
use init_images::init_images;
use menu::MenusPlugin;
use resources::*;
use state::{DisplayState, MoveState};
use std::fs::File;
use std::io::Read;
mod board;
mod components;
mod consts;
mod exit;
mod game;
mod init_images;
mod menu;
mod resources;
mod state;
mod utils;

fn main() {
    App::new()
        .insert_resource(CurrentLevel {
            level_number: 1,
            level_amount: 0,
            level_map_string: "".to_string(),
            is_in_level: false,
        })
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(CurrentSprite(0))
        .init_state::<DisplayState>()
        .init_state::<MoveState>()
        .add_plugins(MenusPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(DisplayPlugin)
        .add_plugins(MovementPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, init_images)
        .add_systems(Update, update_images)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update_images(mut current_sprite: ResMut<CurrentSprite>) {
    let mut file = File::open(PLAYER_TEXTURE_SAVE).unwrap();
    let mut buf = [0_u8; 1];
    file.read_exact(&mut buf).unwrap();
    current_sprite.0 = buf[0] as usize;
}
