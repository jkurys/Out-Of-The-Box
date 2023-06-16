use bevy::prelude::*;

use crate::{consts::*, resources::Images};

pub fn init_images(
    mut images: ResMut<Images>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    images.goal_image = asset_server.load(GOAL_TEXTURE);
    images.ice_image = asset_server.load(ICE_TEXTURE);
    images.warp_image = asset_server.load(WARP_TEXTURE);
    images.tile_image = asset_server.load(TILE_TEXTURE);

    let wall_atlas_texture = asset_server.load(WALL_ATLAS);
    let wall_atlas = TextureAtlas::from_grid(
        wall_atlas_texture,
        Vec2::splat(16.),
        2,
        1,
        Some(Vec2::splat(4.)),
        None,
    );
    images.wall_images = Some(atlases.add(wall_atlas));

    let box_atlas_texture = asset_server.load(BOX_ATLAS);
    let box_atlas = TextureAtlas::from_grid(
        box_atlas_texture,
        Vec2::splat(16.),
        2,
        2,
        Some(Vec2::splat(4.)),
        None,
    );
    images.box_images = Some(atlases.add(box_atlas));

    let hiding_wall_atlas_texture = asset_server.load(HIDING_WALL_ATLAS);
    let hiding_wall_atlas = TextureAtlas::from_grid(
        hiding_wall_atlas_texture,
        Vec2::splat(16.),
        3,
        3,
        Some(Vec2::splat(4.)),
        None,
    );
    images.hidden_wall_images = Some(atlases.add(hiding_wall_atlas));

    let player_atlas_texture = asset_server.load(PLAYER_ATLAS);
    let player_atlas = TextureAtlas::from_grid(
        player_atlas_texture,
        Vec2::splat(16.),
        2,
        3,
        Some(Vec2::splat(4.)),
        None,
    );
    images.player_images = Some(atlases.add(player_atlas));
}
