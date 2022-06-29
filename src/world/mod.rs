pub mod despawner;
pub mod spawner;
pub mod tile;

use bevy::prelude::{App, Plugin};
use iyes_loopless::prelude::ConditionSet;

use crate::app::GameState;

use self::{
    despawner::{despawn_all_terrain, despawn_far_chunk_instruction, DespawnAllTerrain},
    spawner::{
        drawer::draw_chunks_from_instruction,
        instruction::{spawn_chunk_instruction, LoadedChunks},
    },
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // List of Currently Loaded Chunks
            .insert_resource(LoadedChunks::default())
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InGame)
                    .with_system(spawn_chunk_instruction)
                    .with_system(draw_chunks_from_instruction)
                    .with_system(despawn_far_chunk_instruction)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InGame)
                    .run_if_resource_exists::<DespawnAllTerrain>()
                    .with_system(despawn_all_terrain)
                    .into(),
            );
    }
}
