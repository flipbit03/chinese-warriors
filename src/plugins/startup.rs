use bevy::prelude::{App, Plugin, StartupStage};

use crate::systems::resources::{
    asset_reloader::activate_live_asset_reloading, fonts::load_fonts, game::load_game_assets,
    textures::load_textures, timers::load_timers,
};

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_fonts)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_textures)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_timers)
            .add_startup_system_to_stage(StartupStage::Startup, load_game_assets)
            .add_startup_system(activate_live_asset_reloading);
    }
}
