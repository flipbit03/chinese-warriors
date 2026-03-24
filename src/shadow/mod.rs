use bevy::prelude::*;

use crate::app::GameState;

use self::add_shadow::add_shadow_to_character;

pub mod add_shadow;
pub mod structs;

pub struct ShadowsPlugin;

impl Plugin for ShadowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            add_shadow_to_character.run_if(in_state(GameState::InGame)),
        );
    }
}
