pub mod despawner;
pub mod spawner;
pub mod tile;

use bevy::prelude::*;

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
        app.insert_resource(LoadedChunks::default())
            .add_systems(
                Update,
                (
                    spawn_chunk_instruction,
                    draw_chunks_from_instruction,
                    despawn_far_chunk_instruction,
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                despawn_all_terrain
                    .run_if(in_state(GameState::InGame))
                    .run_if(resource_exists::<DespawnAllTerrain>),
            );
    }
}
