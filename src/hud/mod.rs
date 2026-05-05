use bevy::prelude::*;

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
        app.add_systems(OnEnter(GameState::InGame), spawn_hud_text)
            .add_systems(
                Update,
                (update_hud_text, update_window_title)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
