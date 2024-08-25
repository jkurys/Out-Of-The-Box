use background::render_board;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};

use self::border::render_border;
use self::level_background::render_background;
use self::text::{display_level_text, LevelText};
// use crate::consts::*;
use crate::game::GameItem;
use crate::resources::Images;
use crate::state::{DisplayState, MoveState};
use crate::utils::delete_all_components;

use super::movement::is_in_game;

pub mod background;
pub mod glue;
pub mod border;
pub mod floor;
mod level_background;
mod render_2_5_d;
mod text;

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Images>();
        app.add_systems(Startup, window_set_fullscreen);
        app.add_systems(
            OnEnter(DisplayState::Game),
            (render_background, display_level_text),
        );
        app.add_systems(
            OnExit(DisplayState::Game),
            delete_all_components::<LevelText>,
        );
        app.add_systems(
            Update,
            (despawn_board, render_board, render_border)
                .chain()
                .run_if(is_in_game)
                .run_if(in_state(MoveState::Static)),
        );
    }
}

// pub fn render_entity<T>(
//     component: T,
//     commands: &mut Commands,
//     image: Handle<Image>,
//     position: Position,
//     z_index: f32,
// ) -> Entity
// where
//     T: Component,
// {
//     let (x, y) = (
//         position.x as f32 * TILE_WIDTH + position.y as f32 * (101. / 300.) * TILE_WIDTH,
//         position.y as f32 * (TILE_HEIGHT - 3.)
//     );
//     commands
//         .spawn((SpriteBundle {
//             texture: image,
//             transform: Transform::from_xyz(x, y, z_index).with_scale(Vec3::new(
//                 IMAGE_MULTIPLIER,
//                 IMAGE_MULTIPLIER,
//                 1.,
//             )),
//             ..default()
//         },))
//         .insert(component)
//         .insert(GameItem)
//         .id()
// }
//
// pub fn spawn_from_atlas<T>(
//     commands: &mut Commands,
//     atlas_handle: Handle<TextureAtlas>,
//     index: usize,
//     x: i32,
//     y: i32,
//     z_index: f32,
//     component: T,
// ) -> Entity
// where
//     T: Component + Clone,
// {
//     let mut image = TextureAtlasSprite::new(index);
//     image.custom_size = Some(Vec2 { x: TILE_WIDTH * 4.8/3., y: TILE_HEIGHT * 4.8/3.});
//     let (x, y, z) = (
//         (x as f32 - 0.05) * TILE_WIDTH + y as f32 * (101. / 300.) * TILE_WIDTH,
//         (y as f32 + 0.06) * (TILE_HEIGHT - 3.),
//         z_index,
//     );
//     commands
//         .spawn(SpriteSheetBundle {
//             sprite: image,
//             texture_atlas: atlas_handle,
//             transform: Transform::from_xyz(x, y, z),
//             ..default()
//         })
//         .insert((component, GameItem))
//         .id()
// }
//
pub fn despawn_board(query: Query<Entity, With<GameItem>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn window_set_fullscreen(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.get_single_mut().expect("Could not get window");
    window.mode = WindowMode::Fullscreen;
}
