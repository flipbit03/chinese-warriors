use bevy::prelude::{AssetEvent, Assets, Commands, EventReader, Res};

use super::structs::CwConfig;

pub fn live_update_config(
    mut commands: Commands,
    mut asset_events: EventReader<AssetEvent<CwConfig>>,
    asset_collection: Res<Assets<CwConfig>>,
) {
    for asset_event in asset_events.iter() {
        if let Some(h) = match asset_event {
            AssetEvent::Created { handle } => Some(handle),
            AssetEvent::Modified { handle } => Some(handle),
            _ => None,
        } {
            let updated_config = asset_collection.get(h).unwrap();
            println!("Updated Config => {:?}", &updated_config);
            commands.insert_resource(updated_config.clone());
        }
    }
}
