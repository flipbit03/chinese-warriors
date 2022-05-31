use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

use crate::app::GameState;

use self::{
    spawner::{spawn_hud_text, update_hud_text},
    window_title::update_window_title,
};
pub mod spawner;
pub mod window_title;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::InGame, spawn_hud_text)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InGame)
                    .with_system(update_hud_text)
                    .with_system(update_window_title)
                    .into(),
            );
    }
}
