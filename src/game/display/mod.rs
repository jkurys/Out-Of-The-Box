use background::render_board;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};

use self::background::render_border;
use self::text::{display_level_text, LevelText};
use crate::consts::*;
use crate::game::{game_objects::Position, GameItem};
use crate::resources::Images;
use crate::state::{DisplayState, MoveState};
use crate::utils::delete_all_components;

use super::movement::is_in_game;

pub mod background;
mod render_2_5_d;
mod text;

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Images>();
        // app.add_startup_system(window_set_fullscreen);
        app.add_system(display_level_text.in_schedule(OnEnter(DisplayState::Game)));
        app.add_system(delete_all_components::<LevelText>.in_schedule(OnExit(DisplayState::Game)));
        app.add_systems(
            (despawn_board, render_board, render_border)
                .chain()
                .distributive_run_if(is_in_game)
                .in_set(OnUpdate(MoveState::Static)),
        );
    }
}

pub fn render_entity<T>(
    component: T,
    commands: &mut Commands,
    image: Handle<Image>,
    position: Position,
    z_index: f32,
) -> Entity
where
    T: Component,
{
    let (x, y) = (position.x as f32 * TILE_SIZE, position.y as f32 * TILE_SIZE);
    commands
        .spawn((SpriteBundle {
            texture: image,
            transform: Transform::from_xyz(x, y, z_index).with_scale(Vec3::new(
                TILE_SIZE / IMAGE_SIZE,
                TILE_SIZE / IMAGE_SIZE,
                1.,
            )),
            ..default()
        },))
        .insert(component)
        .insert(GameItem)
        .id()
}

pub fn spawn_from_atlas<T>(
    commands: &mut Commands,
    atlas_handle: Handle<TextureAtlas>,
    index: usize,
    x: i32,
    y: i32,
    component: T,
) -> Entity
where
    T: Component + Clone,
{
    let mut image = TextureAtlasSprite::new(index);
    image.custom_size = Some(Vec2::splat(TILE_SIZE));
    let (x, y, z) = (x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, FLOOR_Z_INDEX);
    commands
        .spawn(SpriteSheetBundle {
            sprite: image,
            texture_atlas: atlas_handle.clone(),
            transform: Transform::from_xyz(x, y, z),
            ..default()
        })
        .insert((component, GameItem))
        .id()
}

pub fn despawn_board(query: Query<Entity, With<GameItem>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn window_set_fullscreen(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.get_single_mut().expect("Could not get window");
    window.mode = WindowMode::Fullscreen;
}
