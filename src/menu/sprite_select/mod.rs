use bevy::prelude::*;

use crate::{utils::delete_all_components, exit::handle_esc, state::DisplayState};

use self::{setup::setup_sprite_select, handle_click::{handle_sprite_click, SpriteSelectItem}};

mod styles;
mod handle_click;
mod setup;

pub struct SpriteSelectPlugin;

impl Plugin for SpriteSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_sprite_select.in_schedule(OnEnter(DisplayState::SpriteSelect)))
            .add_systems((handle_sprite_click, handle_esc).in_set(OnUpdate(DisplayState::SpriteSelect)))
            .add_system(delete_all_components::<SpriteSelectItem>.in_schedule(OnExit(DisplayState::SpriteSelect)));
    }
}