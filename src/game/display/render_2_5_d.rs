use bevy::prelude::*;

use crate::{
    consts::*,
    game::GameItem,
};

pub fn render_object<T>(
    commands: &mut Commands,
    atlas_handle: Handle<TextureAtlas>,
    indices: (usize, usize, usize),
    x: i32,
    y: i32,
    z: i32,
    z_index_mod: f32,
    component: T,
) -> [Entity; 3]
where
    T: Component + Clone,
{
    let (bottom_index, top_index, side_index) = indices;
    let mut higher_image = TextureAtlasSprite::new(top_index);
    let mut lower_image = TextureAtlasSprite::new(bottom_index);
    let mut side_image = TextureAtlasSprite::new(side_index);
    higher_image.custom_size = Some(Vec2 { x: TILE_WIDTH * 4.8/3., y: TILE_HEIGHT * 4.8/3. });
    lower_image.custom_size = Some(Vec2 { x: TILE_WIDTH * 4.8/3., y: TILE_HEIGHT * 4.8/3. });
    side_image.custom_size = Some(Vec2 { x: TILE_WIDTH * 4.8/3., y: TILE_HEIGHT * 4.8/3. });
    let (upper_x, upper_y, upper_z) = (
        (x as f32) * TILE_WIDTH + (y as f32 * (101./300.) * TILE_WIDTH),
        (y as f32 + 0.7) * (TILE_HEIGHT - 3.) + ((z - 1) as f32 * TILE_FRONT_HEIGHT),
        UPPER_HALF_OBJECT_Z_INDEX + z_index_mod + (z * 2) as f32,
    );
    let (lower_x, lower_y, lower_z) = (
        (x as f32) * TILE_WIDTH + (y as f32 * (101./300.) * TILE_WIDTH),
        (y as f32 - 0.3) * (TILE_HEIGHT - 3.) + ((z - 1) as f32 * TILE_FRONT_HEIGHT),
        LOWER_HALF_OBJECT_Z_INDEX + z_index_mod + (z * 2) as f32,
    );
    let (side_x, side_y, side_z) = (
        (x as f32) * TILE_WIDTH + (y as f32 * (101./300.) * TILE_WIDTH),
        (y as f32 + 0.7) * (TILE_HEIGHT - 3.) + ((z - 1) as f32 * TILE_FRONT_HEIGHT),
        LOWER_HALF_OBJECT_Z_INDEX + z_index_mod + (z * 2) as f32,
    );
    let entity1 = commands
        .spawn(SpriteSheetBundle {
            sprite: higher_image,
            texture_atlas: atlas_handle.clone(),
            transform: Transform::from_xyz(upper_x, upper_y, upper_z),
            ..default()
        })
        .insert((component.clone(), GameItem))
        .id();
    let entity2 = commands
        .spawn(SpriteSheetBundle {
            sprite: lower_image,
            texture_atlas: atlas_handle.clone(),
            transform: Transform::from_xyz(lower_x, lower_y, lower_z + (0.01 * x as f32) - (0.01 * y as f32)),
            ..default()
        })
        .insert((component.clone(), GameItem))
        .id();
    let entity3 = commands
        .spawn(SpriteSheetBundle {
            sprite: side_image,
            texture_atlas: atlas_handle.clone(),
            transform: Transform::from_xyz(side_x, side_y, side_z + (0.01 * x as f32) - (0.01 * y as f32)),
            ..default()
        })
        .insert((component.clone(), GameItem))
        .id();

    [entity1, entity2, entity3]
}

pub fn render_object_with_sticker<T>(
    commands: &mut Commands,
    atlas_handle: Handle<TextureAtlas>,
    indices: (usize, usize, usize),
    sticker_index: usize,
    x: i32,
    y: i32,
    z: i32,
    z_index_mod: f32,
    component: T,
) -> [Entity; 4]
where
    T: Component + Clone,
{
    let (bottom_index, top_index, side_index) = indices;
    let [entity1, entity2, entity3] = render_object(
        commands,
        atlas_handle.clone(),
        (bottom_index, top_index, side_index),
        x,
        y,
        z,
        z_index_mod,
        component.clone(),
    );

    let entity4 = render_sticker(
        commands,
        sticker_index,
        x,
        y,
        z,
        atlas_handle,
        component,
        UPPER_HALF_STICKER_Z_INDEX + z_index_mod,
    );
    [entity1, entity2, entity3, entity4]
}

pub fn render_sticker<T>(
    commands: &mut Commands,
    sticker_index: usize,
    x: i32,
    y: i32,
    z: i32,
    atlas_handle: Handle<TextureAtlas>,
    component: T,
    z_index: f32,
) -> Entity
where
    T: Component + Clone,
{
    let mut sticker_image = TextureAtlasSprite::new(sticker_index);
    sticker_image.custom_size = Some(Vec2 { x: TILE_WIDTH * (4.8/3.), y: TILE_HEIGHT * (4.8/3.) });
    let (sticker_x, sticker_y, sticker_z) =
    (

        (x as f32) * TILE_WIDTH + (y as f32 * (101./300.) * TILE_WIDTH),
        (y as f32 + 0.7) * (TILE_HEIGHT - 3.) + ((z - 1) as f32 * TILE_FRONT_HEIGHT),
        (z_index + (z * 2) as f32),
    );
    commands
        .spawn(SpriteSheetBundle {
            sprite: sticker_image,
            texture_atlas: atlas_handle,
            transform: Transform::from_xyz(sticker_x, sticker_y, sticker_z),
            ..default()
        })
        .insert((component, GameItem))
        .id()
}
