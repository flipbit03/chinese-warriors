use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;

static GURI_ASEPRITE_PATH: &str = "art/hero/guri/Guri.aseprite";

/// Pre-loaded handle to the Guri aseprite file.
#[derive(Resource)]
pub struct GuriAssets {
    pub aseprite: Handle<Aseprite>,
}

pub fn load_guri_aseprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Loading Guri.aseprite...");
    commands.insert_resource(GuriAssets {
        aseprite: asset_server.load(GURI_ASEPRITE_PATH),
    });
}
