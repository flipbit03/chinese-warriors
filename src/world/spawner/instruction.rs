use crate::assets::config::structs::CwConfig;
use crate::world::tile::builder::WorldBuilder;
use crate::world::tile::chunk::WorldChunk;
use crate::world::tile::position::TilePosition;
use crate::world::tile::visibility::{get_screen_rect, get_visible_chunks};

use bevy::prelude::Transform;
use bevy::prelude::*;
use bevy::render::camera::Camera2d;
use bevy::utils::HashSet;
use itertools::Itertools;

#[derive(Default)]
pub struct LoadedChunks {
    pub chunk_set: HashSet<TilePosition>,
}

pub fn spawn_chunk_instruction(
    mut commands: Commands,
    camera_query: Query<(&Camera, &Transform, &OrthographicProjection), With<Camera2d>>,
    world_builder: Res<WorldBuilder>,
    config: Res<CwConfig>,
    existing_chunks_query: Query<&WorldChunk, With<WorldChunk>>,
) {
    let (_camera, camera_transform, camera_projection) = camera_query.single();

    let screen_dimensions = get_screen_rect(
        camera_transform,
        camera_projection,
        camera_projection.scale * 2.0,
    );

    let existing_chunk_positions = existing_chunks_query
        .iter()
        .map(|wc| wc.position.clone())
        .collect_vec();

    let chunks_to_spawn = get_visible_chunks(
        screen_dimensions,
        config.world.tile_size,
        config.world.tile_scale,
    )
    .filter(|visible_chunk_position| {
        !(&existing_chunk_positions).contains(visible_chunk_position)
    })
    .collect_vec();

    // Spawn Chunk Instructions (another system will pick those up for the actual drawing)
    chunks_to_spawn
        .into_iter()
        .for_each(|chunk_position_to_spawn| {
            commands
                .spawn_bundle(world_builder.get_chunk(&chunk_position_to_spawn))
                .insert(WorldChunk {
                    position: chunk_position_to_spawn,
                });
        });
}
