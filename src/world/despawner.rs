use crate::assets::config::structs::CwConfig;

pub struct DespawnAllTerrain;

use super::tile::{
    chunk::{TilesInChunk, WorldChunk},
    visibility::{get_screen_rect, get_visible_chunks},
};

use bevy::{prelude::*, render::camera::Camera2d};
use itertools::Itertools;

pub fn despawn_far_chunk_instruction(
    mut commands: Commands,
    camera_query: Query<(&Transform, &OrthographicProjection), With<Camera2d>>,
    config: Res<CwConfig>,
    existing_chunks_query: Query<
        (Entity, &WorldChunk, &TilesInChunk),
        With<TilesInChunk>,
    >,
) {
    let (camera_transform, camera_projection) = camera_query.single();

    let screen_dimensions = get_screen_rect(
        camera_transform,
        camera_projection,
        camera_projection.scale * 2.0,
    );

    let visible_chunks = get_visible_chunks(
        screen_dimensions,
        config.world.tile_size,
        config.world.tile_scale,
    )
    .collect_vec();

    existing_chunks_query
        .iter()
        .filter(|(_, wc, _)| !visible_chunks.contains(&wc.position))
        .for_each(|(chunk_entity, _, tin)| {
            commands.entity(chunk_entity).despawn();
            if let Some(debug_chunk_entity) = tin.chunk_debug_entity {
                commands.entity(debug_chunk_entity).despawn();
            }

            for tile_entity in tin.tile_entities {
                commands.entity(tile_entity).despawn_recursive();
            }
        });
}

pub fn despawn_all_terrain(
    mut commands: Commands,
    tile_query: Query<(Entity, &TilesInChunk), With<WorldChunk>>,
) {
    for (chunk_entity, tin) in tile_query.iter() {
        commands.entity(chunk_entity).despawn();
        if let Some(debug_chunk_entity) = tin.chunk_debug_entity {
            commands.entity(debug_chunk_entity).despawn();
        }

        for tile_entity in tin.tile_entities {
            commands.entity(tile_entity).despawn_recursive();
        }
    }

    commands.remove_resource::<DespawnAllTerrain>();
}
