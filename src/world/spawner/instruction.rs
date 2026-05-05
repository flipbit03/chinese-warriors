use crate::assets::config::structs::CwConfig;
use crate::world::tile::builder::WorldBuilder;
use crate::world::tile::chunk::WorldChunk;
use crate::world::tile::position::TilePosition;
use crate::world::tile::visibility::{get_screen_rect, get_visible_chunks};

use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use itertools::Itertools;

#[derive(Default, Resource)]
pub struct LoadedChunks {
    pub chunk_set: HashSet<TilePosition>,
}

pub fn spawn_chunk_instruction(
    mut commands: Commands,
    camera_query: Query<(&Camera, &Transform, &Projection), With<Camera2d>>,
    window_query: Query<&Window>,
    world_builder: Res<WorldBuilder>,
    config: Res<CwConfig>,
    existing_chunks_query: Query<&WorldChunk, With<WorldChunk>>,
) {
    let Ok((_camera, camera_transform, projection)) = camera_query.single() else {
        return;
    };
    let Projection::Orthographic(camera_projection) = projection else {
        return;
    };
    let Ok(window) = window_query.single() else {
        return;
    };

    let screen_dimensions = get_screen_rect(
        camera_transform,
        camera_projection,
        window,
        1.3,
    );

    let existing_chunk_positions = existing_chunks_query
        .iter()
        .map(|wc| wc.position)
        .collect_vec();

    let chunks_to_spawn = get_visible_chunks(
        screen_dimensions,
        config.world.tile_size,
        config.world.tile_scale,
    )
    .filter(|visible_chunk_position| {
        !existing_chunk_positions.contains(visible_chunk_position)
    })
    .collect_vec();

    chunks_to_spawn
        .into_iter()
        .for_each(|chunk_position_to_spawn| {
            commands
                .spawn(world_builder.get_chunk(&chunk_position_to_spawn))
                .insert(WorldChunk {
                    position: chunk_position_to_spawn,
                });
        });
}
