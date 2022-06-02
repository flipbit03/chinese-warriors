use bevy::prelude::{AssetServer, Commands, Handle, Res};
pub mod default;
pub mod live_updater;
pub mod structs;
use self::structs::CwConfig;

/// Setup System
pub fn load_config_save_handle(mut commands: Commands, server: Res<AssetServer>) {
    // Load the config as a Handle<CwConfig> with asset server (for live reload)
    let config_handle: Handle<CwConfig> = server.load("config/world.config.ron");
    commands.insert_resource(config_handle);
}
