use bevy::prelude::*;
use consts::*;
use init_images::init_images;
use plugin::MainPlugin;
use resources::*;
use std::fs::File;
use std::io::Read;

mod board;
mod components;
mod consts;
mod exit;
mod game;
mod init_images;
mod menu;
mod plugin;
mod resources;
mod state;
#[cfg(test)]
mod tests;
mod utils;

fn main() {
    App::new().add_plugins(MainPlugin).run();
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
