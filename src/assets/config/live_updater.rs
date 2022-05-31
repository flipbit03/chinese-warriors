use bevy::prelude::{AssetEvent, Assets, Commands, EventReader, Res};

use crate::{assets::game::generate_tile_builder_resource_from_config, world::despawner::DespawnAllTerrain};

use super::structs::CwConfig;

pub fn live_update_main_config(
    mut commands: Commands,
    mut asset_events: EventReader<AssetEvent<CwConfig>>,
    asset_collection: Res<Assets<CwConfig>>,
) {
    let mut updated_config: Option<CwConfig> = None;

    for asset_event in asset_events.iter() {
        if let Some(h) = match asset_event {
            AssetEvent::Created { handle } => Some(handle),
            AssetEvent::Modified { handle } => Some(handle),
            _ => None,
        } {
            updated_config = Some(asset_collection.get(h).unwrap().to_owned());
        }
    }

    match updated_config {
        Some(c) => {
            println!("Updated Config => {:?}", &c);
            generate_tile_builder_resource_from_config(&mut commands, &c);
            commands.insert_resource(DespawnAllTerrain);
            commands.insert_resource(c.clone());
        }
        None => (),
    };
}
