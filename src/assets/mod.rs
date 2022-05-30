use bevy::prelude::{App, Plugin, StartupStage};
use bevy_asset_ron::RonAssetPlugin;

use self::{
    asset_reloader::activate_live_asset_reloading,
    config::{live_updater::live_update_config, load_initial_config, structs::CwConfig},
    fonts::load_fonts,
    game::load_game_assets,
    textures::load_textures,
    timers::load_timers,
};

pub mod asset_reloader;
pub mod config;
pub mod fonts;
pub mod game;
pub mod textures;
pub mod timers;
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<CwConfig>::new(&["config"]))
            .add_startup_system_to_stage(StartupStage::PreStartup, activate_live_asset_reloading)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_initial_config)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_fonts)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_textures)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_timers)
            .add_startup_system_to_stage(StartupStage::Startup, load_game_assets)
            .add_system(live_update_config);
    }
}
