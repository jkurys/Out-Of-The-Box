use background::render_board;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};

use self::border::render_border;
use self::level_background::render_background;
use self::resources::{ButtonAnimationTimer, ButtonState};
use self::text::{display_level_text, LevelText};
use crate::consts::BUTTON_PRESS_TEXTURE;
use crate::game::GameItem;
use crate::resources::Images;
use crate::state::{DisplayState, MoveState};
use crate::utils::delete_all_components;

use super::movement::is_in_game;
use super::movement::resources::DisplayButton;

pub mod background;
pub mod glue;
pub mod border;
pub mod floor;
mod level_background;
pub mod render_2_5_d;
mod text;
mod resources;

#[derive(Component)]
pub struct ButtonPopup;

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
            (despawn_board, render_board, render_border, update_button)
                .chain()
                .run_if(is_in_game)
                .run_if(in_state(MoveState::Static)),
        );
        app.insert_resource(ButtonAnimationTimer(Timer::from_seconds(
            0.2,
            TimerMode::Once,
        )));
        app.insert_resource(ButtonState(true));
    }
}

pub fn update_button(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut timer: ResMut<ButtonAnimationTimer>,
    mut button_state: ResMut<ButtonState>,
    mut commands: Commands,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
    query: Query<Entity, With<ButtonPopup>>,
    display_button: Res<DisplayButton>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    if !display_button.0 {
        return;
    }

    timer.0.tick(time.delta());
    let button_press = asset_server.load(BUTTON_PRESS_TEXTURE);
    if timer.0.finished() {
        button_state.0 = !button_state.0;
        timer.0.reset();
    }
    commands.spawn((SpriteBundle {
        texture: button_press,
        transform: Transform {
            translation: Vec3 {
                x: 150.,
                y: -500.,
                z: 200.,
            },
            ..default()
        },
        ..default()
        
    }, TextureAtlas {
        layout: atlases.add(TextureAtlasLayout::from_grid(    
            UVec2 { x: 480, y: 480 },
            2,
            2,
            Some(UVec2 { x: 20, y: 20 }),
            None,
        )),
        index: button_state.0 as usize,
    })).insert(ButtonPopup);

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
