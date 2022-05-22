use bevy::prelude::{App, Plugin, StartupStage};

use crate::systems::hud::{
    spawner::{generate_ui_camera, spawn_hud_text},
    update_hud,
    window_title::window_title_resolution,
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, generate_ui_camera)
            .add_startup_system(spawn_hud_text)
            .add_system(update_hud)
            .add_system(window_title_resolution);
    }
}
