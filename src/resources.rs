use bevy::prelude::*;

use crate::consts::*;
use std::io::Read;
use std::fs::File;

#[derive(Resource)]
pub struct CurrentLevel {
    pub level_number: usize,
    pub level_map_str: &'static str,
}

#[derive(Resource)]
pub struct Images {
    pub goal_image: Handle<Image>,
    pub player_image: Handle<Image>,
    pub box_image: Handle<Image>,
    pub box_on_goal_image: Handle<Image>,
    pub wall_image: Handle<Image>,
    pub tile_image: Handle<Image>,
    pub ice_image: Handle<Image>,
    pub warp_image: Handle<Image>,
    pub button_image: Handle<Image>,
    pub hidden_wall_image: Handle<Image>,
    pub shown_hidden_wall_image: Handle<Image>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("Asset server not found in world");
        let mut file = File::open("assets/saves/save.txt").unwrap();
        let mut buf = [0 as u8; 1];
        file.read(&mut buf).unwrap();
        let player_image = asset_server.load(PLAYER_TEXTURES[buf[0] as usize]);
        let box_image = asset_server.load(BOX_TEXTURE);
        let wall_image = asset_server.load(WALL_TEXTURE);
        let goal_image = asset_server.load(GOAL_TEXTURE);
        let box_on_goal_image = asset_server.load(BOX_ON_GOAL_TEXTURE);
        let tile_image = asset_server.load(TILE_TEXTURE);
        let ice_image = asset_server.load(ICE_TEXTURE);
        let warp_image = asset_server.load(WARP_TEXTURE);
        let button_image = asset_server.load(BUTTON_TEXTURE);
        let hidden_wall_image = asset_server.load(HIDDEN_WALL_TEXTURE);
        let shown_hidden_wall_image = asset_server.load(SHOWN_HIDDEN_WALL_TEXTURE);

        Images {
            player_image,
            box_image,
            wall_image,
            goal_image,
            box_on_goal_image,
            tile_image,
            ice_image,
            warp_image,
            button_image,
            hidden_wall_image,
            shown_hidden_wall_image,
        }
    }
}
