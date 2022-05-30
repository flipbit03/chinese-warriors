use bevy::prelude::*;

use self::{
    spawner::{generate_ui_camera, spawn_hud_text, update_hud_text},
    window_title::update_window_title,
};
pub mod spawner;
pub mod window_title;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, generate_ui_camera)
            .add_startup_system(spawn_hud_text)
            .add_system(update_hud_text)
            .add_system(update_window_title);
    }
}
