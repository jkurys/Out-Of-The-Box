use bevy::prelude::*;

use crate::{
    consts::{
        LOWER_HALF_OBJECT_Z_INDEX, TILE_SIZE, UPPER_HALF_OBJECT_Z_INDEX, UPPER_HALF_STICKER_Z_INDEX,
    },
    game::GameItem,
};

pub fn render_object<T>(
    commands: &mut Commands,
    atlas_data: (Handle<TextureAtlasLayout>, Handle<Image>),
    indices: (usize, usize),
    x: i32,
    y: i32,
    component: T,
) -> [Entity; 2]
where
    T: Component + Clone,
{
    let (atlas_handle, texture) = atlas_data;
    let (bottom_index, top_index) = indices;
    let higher_atlas = TextureAtlas { index: top_index, layout: atlas_handle.clone() };
    let lower_atlas = TextureAtlas { index: bottom_index, layout: atlas_handle.clone() };
    let mut lower_sprite = Sprite::default();
    let mut higher_sprite = Sprite::default();
    higher_sprite.custom_size = Some(Vec2::splat(TILE_SIZE + 0.5));
    lower_sprite.custom_size = Some(Vec2::splat(TILE_SIZE + 0.5));
    let (upper_x, upper_y, upper_z) = (
        x as f32 * TILE_SIZE,
        (y as f32 + 0.24) * TILE_SIZE,
        UPPER_HALF_OBJECT_Z_INDEX,
    );
    let (lower_x, lower_y, lower_z) = (
        x as f32 * TILE_SIZE,
        (y as f32 - 0.375) * TILE_SIZE,
        LOWER_HALF_OBJECT_Z_INDEX,
    );
    let entity1 = commands
        .spawn(SpriteSheetBundle {
            sprite: higher_sprite,
            atlas: higher_atlas,
            texture: texture.clone(),
            transform: Transform::from_xyz(upper_x, upper_y, upper_z),
            ..default()
        })
        .insert((component.clone(), GameItem))
        .id();
    let entity2 = commands
        .spawn(SpriteSheetBundle {
            sprite: lower_sprite,
            atlas: lower_atlas,
            texture,
            transform: Transform::from_xyz(lower_x, lower_y, lower_z),
            ..default()
        })
        .insert((component, GameItem))
        .id();
    [entity1, entity2]
}

pub fn render_object_with_sticker<T>(
    commands: &mut Commands,
    atlas_data: (Handle<TextureAtlasLayout>, Handle<Image>),
    indices: (usize, usize),
    sticker_index: usize,
    x: i32,
    y: i32,
    component: T,
) -> [Entity; 3]
where
    T: Component + Clone,
{
    let (atlas_handle, texture) = atlas_data.clone();
    let (bottom_index, top_index) = indices;
    let [entity1, entity2] = render_object(
        commands,
        atlas_data.clone(),
        (bottom_index, top_index),
        x,
        y,
        component.clone(),
    );
    let atlas = TextureAtlas { index: sticker_index, layout: atlas_handle };
    let mut sticker_sprite = Sprite::default();
    sticker_sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    let (sticker_x, sticker_y, sticker_z) = (
        x as f32 * TILE_SIZE,
        (y as f32 + 0.24) * TILE_SIZE,
        UPPER_HALF_STICKER_Z_INDEX,
    );
    let entity3 = commands
        .spawn(SpriteSheetBundle {
            sprite: sticker_sprite,
            atlas,
            texture,
            transform: Transform::from_xyz(sticker_x, sticker_y, sticker_z),
            ..default()
        })
        .insert((component, GameItem))
        .id();
    [entity1, entity2, entity3]
}
