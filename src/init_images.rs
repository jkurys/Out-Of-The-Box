use bevy::prelude::*;

use crate::{consts::*, resources::Images};

pub fn init_images(
    mut images: ResMut<Images>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let wall_atlas_texture = asset_server.load(WALL_ATLAS);
    let wall_atlas = TextureAtlasLayout::from_grid(
        Vec2::splat(16.),
        2,
        1,
        Some(Vec2::splat(4.)),
        None,
    );
    images.wall_images = Some((atlases.add(wall_atlas), wall_atlas_texture));

    let box_atlas_texture = asset_server.load(BOX_ATLAS);
    let box_atlas = TextureAtlasLayout::from_grid(
        Vec2::splat(16.),
        2,
        2,
        Some(Vec2::splat(4.)),
        None,
    );
    images.box_images = Some((atlases.add(box_atlas), box_atlas_texture));

    let hiding_wall_atlas_texture = asset_server.load(HIDING_WALL_ATLAS);
    let hiding_wall_atlas = TextureAtlasLayout::from_grid(
        Vec2::splat(16.),
        3,
        3,
        Some(Vec2::splat(4.)),
        None,
    );
    images.hidden_wall_images = Some((atlases.add(hiding_wall_atlas), hiding_wall_atlas_texture));

    let player_atlas_texture = asset_server.load(PLAYER_ATLAS);
    let player_atlas = TextureAtlasLayout::from_grid(
        Vec2::splat(16.),
        4 * 2,
        3,
        Some(Vec2::splat(4.)),
        None,
    );
    images.player_images = Some((atlases.add(player_atlas), player_atlas_texture));
    let turtle_images_texture = asset_server.load(TURTLE_ATLAS);
    let turtle_atlas = TextureAtlasLayout::from_grid(
        Vec2::splat(16.),
        4,
        5,
        Some(Vec2::splat(4.)),
        None,
    );
    images.turtle_images = Some((atlases.add(turtle_atlas), turtle_images_texture));
}
