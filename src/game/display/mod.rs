use background::render_board;
use bevy::prelude::*;

use self::background::render_border;
use self::text::{display_level_text, LevelText};
use crate::consts::*;
use crate::game::movement::resources::AnimationTimer;
use crate::game::{game_objects::Position, GameItem};
use crate::labels::Labels;
use crate::resources::Images;
use crate::state::{DisplayState, GameState, Move};
use crate::utils::delete_all_components;

pub mod background;
mod text;
pub mod print_board;

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Images>();
        app.add_startup_system(window_set_fullscreen);

        app.add_system_set(SystemSet::on_enter(DisplayState::Game).with_system(display_level_text));

        app.add_system_set(
            SystemSet::on_exit(DisplayState::Game).with_system(delete_all_components::<LevelText>),
        );
        app.add_system_set(
            SystemSet::on_update(GameState(Some(Move::Static)))
                .label(Labels::Display)
                .with_system(despawn_board.before(render_board).before(render_border))
                .with_system(render_board)
                .with_system(render_border),
        );
    }
}

//render an object with a given image and position
// pub fn render_entity<T>(
//     component: T,
//     parent: &mut ChildBuilder,
//     image: Handle<Image>,
//     position: Position,
//     z_index: f32,
// ) -> Entity
pub fn render_entity<T>(
    component: T,
    parent: &mut Commands,
    image: Handle<Image>,
    position: Position,
    z_index: f32,
) -> Entity
where
    T: Component,
{
    parent.spawn(SpriteBundle {
        texture: image,

        ..default()
    }).insert(component)
        .insert(GameItem)
        .id()
    // parent
    //     .spawn((ImageBundle {
    //         image: UiImage(image),
    //         style: Style {
    //             size: Size {
    //                 height: Val::Px(50.),
    //                 width: Val::Px(50.),
    //             },
    //             ..default()
    //         },
    //         transform: Transform::from_xyz(
    //             position.x as f32 * TILE_SIZE,
    //             position.y as f32 * TILE_SIZE,
    //             z_index,
    //         ).with_scale(Vec3 { x: IMAGE_SIZE/TILE_SIZE, y: IMAGE_SIZE/TILE_SIZE, z: 1. }),
    //         ..default()
    //     },))
    //     .insert(component)
    //     .insert(GameItem)
    //     .id()

        // parent.spawn(SpriteBundle {
        //     texture: image,
        //     transform: Transform::from_xyz( position.x as f32 * TILE_SIZE,
        //         position.y as f32 * TILE_SIZE,
        //         z_index,
        //     ),
        //     ..default()
        // })
        // .insert(GameItem)
        // .insert(component)
        // .id()
}

pub fn despawn_board(
    query: Query<Entity, With<GameItem>>,
    mut commands: Commands,
    timer: Res<AnimationTimer>,
) {
    if !timer.0.finished() && timer.0.elapsed_secs() != 0. {
        //so the board doesn't re-render
        return;
    }
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn window_set_fullscreen(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_mode(WindowMode::BorderlessFullscreen);
}
