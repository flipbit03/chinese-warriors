use bevy::prelude::*;

pub fn activate_live_asset_reloading(asset_server: Res<AssetServer>) {
    asset_server.watch_for_changes().unwrap();
}