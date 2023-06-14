use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::consts::*;
use crate::state::DisplayState;
use std::fs::File;
use std::io::Read;

#[derive(Resource)]
pub struct CurrentLevel {
    pub level_number: usize,
    pub level_map_string: String,
    pub level_amount: usize,
}

#[derive(Resource)]
pub struct Images {
    pub goal_image: Handle<Image>,
    pub player_images: [Handle<Image>; 2],
    pub box_images: [Handle<Image>; 2],
    pub box_on_goal_images: [Handle<Image>; 2],
    pub wall_images: [Handle<Image>; 2],
    pub tile_image: Handle<Image>,
    pub ice_image: Handle<Image>,
    pub warp_image: Handle<Image>,
    pub button_images: [Handle<Image>; 3],
    pub hidden_wall_images: [Handle<Image>; 3],
    pub shown_hidden_wall_images: [[Handle<Image>; 2]; 3],
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("Asset server not found in world");
        let file_res = File::open(PLAYER_TEXTURE_SAVE);
        let mut buf = [0_u8; 1];
        if let Ok(mut file) = file_res {
            file.read_exact(&mut buf).unwrap();
        }
        let player_images = [asset_server.load(LOWER_PLAYER_TEXTURES[buf[0] as usize]), asset_server.load(PLAYER_TEXTURES[buf[0] as usize])];
        let box_images = [asset_server.load(LOWER_BOX_TEXTURE), asset_server.load(HIGHER_BOX_TEXTURE)];
        let wall_images = [asset_server.load(LOWER_WALL_TEXTURE), asset_server.load(HIGHER_WALL_TEXTURE)];
        let goal_image = asset_server.load(GOAL_TEXTURE);
        let box_on_goal_images = [asset_server.load(LOWER_BOX_TEXTURE), asset_server.load(HIGHER_BOX_ON_GOAL_TEXTURE)];
        let tile_image = asset_server.load(TILE_TEXTURE);
        let ice_image = asset_server.load(ICE_TEXTURE);
        let warp_image = asset_server.load(WARP_TEXTURE);
        let button_images = [
            asset_server.load(BUTTON_TEXTURES[0]),
            asset_server.load(BUTTON_TEXTURES[1]),
            asset_server.load(BUTTON_TEXTURES[2])
        ];
        let hidden_wall_images = [asset_server.load(HIDDEN_WALL_TEXTURES[0]), asset_server.load(HIDDEN_WALL_TEXTURES[1]), asset_server.load(HIDDEN_WALL_TEXTURES[2])];
        let shown_hidden_wall_images = [
            [asset_server.load(LOWER_SHOWN_HIDDEN_WALL_TEXTURES[0]), asset_server.load(HIGHER_SHOWN_HIDDEN_WALL_TEXTURES[0])],
            [asset_server.load(LOWER_SHOWN_HIDDEN_WALL_TEXTURES[1]), asset_server.load(HIGHER_SHOWN_HIDDEN_WALL_TEXTURES[1])],
            [asset_server.load(LOWER_SHOWN_HIDDEN_WALL_TEXTURES[2]), asset_server.load(HIGHER_SHOWN_HIDDEN_WALL_TEXTURES[2])],
        ];

        Images {
            player_images,
            box_images,
            wall_images,
            goal_image,
            box_on_goal_images,
            tile_image,
            ice_image,
            warp_image,
            button_images,
            hidden_wall_images,
            shown_hidden_wall_images,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MapSize {
    pub height: u32,
    pub width: u32,
}

#[derive(Resource)]
pub struct StateStack(pub Vec<DisplayState>);

impl Default for StateStack {
    fn default() -> Self {
        Self(vec![DisplayState::MainMenu])
    }
}
