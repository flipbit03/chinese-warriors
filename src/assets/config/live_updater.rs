use bevy::prelude::*;

use crate::{
    assets::game::generate_tile_builder_resource_from_config,
    world::despawner::DespawnAllTerrain,
};

use super::structs::CwConfig;

pub fn live_update_main_config(
    mut commands: Commands,
    mut asset_events: MessageReader<AssetEvent<CwConfig>>,
    asset_collection: Res<Assets<CwConfig>>,
) {
    let mut updated_config: Option<CwConfig> = None;

    for asset_event in asset_events.read() {
        let id = match asset_event {
            AssetEvent::Added { id } | AssetEvent::Modified { id } => Some(id),
            _ => None,
        };
        if let Some(id) = id {
            if let Some(config) = asset_collection.get(*id) {
                updated_config = Some(config.clone());
            }
        }
    }

    if let Some(updated_config) = updated_config {
        info!("Updated Config!");
        generate_tile_builder_resource_from_config(&mut commands, &updated_config);
        commands.insert_resource(DespawnAllTerrain);
        commands.insert_resource(updated_config);
    }
}
