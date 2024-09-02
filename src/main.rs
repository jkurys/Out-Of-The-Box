use bevy::prelude::*;
// use bevy::render::texture::{ImageSampler, ImageSamplerDescriptor};
use consts::*;
use init_images::init_images;
use plugin::MainPlugin;
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
mod plugin;
mod resources;
mod state;
#[cfg(test)]
mod tests;
mod utils;

fn main() {
    App::new()
        .add_plugins(MainPlugin)
        .init_state::<MoveState>()
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

// fn spritemap_fix(mut ev_asset: EventReader<AssetEvent<Image>>, mut assets: ResMut<Assets<Image>>) {
//     for ev in ev_asset.read() {
//         NOTE: removed the sharp image for now
//         if let AssetEvent::Added { id } = ev {
//             if let Some(texture) = assets.get_mut(id) {
//                 ImageSamplerDescriptor::nearest();
//             }
//         }
//     }
// }
