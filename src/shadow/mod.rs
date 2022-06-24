use bevy::prelude::{App, Plugin};
use iyes_loopless::prelude::ConditionSet;

use crate::app::GameState;

use self::add_shadow::add_shadow_to_character;

pub mod add_shadow;
pub mod structs;

pub struct ShadowsPlugin;

impl Plugin for ShadowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(add_shadow_to_character)
                .into(),
        );
    }
}
