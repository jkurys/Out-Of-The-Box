use bevy::prelude::*;
use bevy::render::texture::ImageSampler;
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
        })
        .insert_resource(CurrentSprite(0))
        .add_state::<DisplayState>()
        .add_state::<MoveState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(MenusPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(DisplayPlugin)
        .add_plugin(MovementPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(init_images)
        .add_system(update_images)
        .add_system(spritemap_fix)
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

fn spritemap_fix(mut ev_asset: EventReader<AssetEvent<Image>>, mut assets: ResMut<Assets<Image>>) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                if let Some(texture) = assets.get_mut(&handle) {
                    texture.sampler_descriptor = ImageSampler::nearest()
                }
            }
            _ => {}
        }
    }
}
