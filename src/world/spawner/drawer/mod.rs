use crate::assets::config::structs::CwConfig;
use crate::assets::textures::TerrainTextures;
use crate::world::spawner::drawer::draw_functions::{
    draw_debug_chunk_grid, draw_single_tile,
};
use crate::world::tile::chunk::{
    TilesInChunk, WorldChunk, WorldChunkInstruction, TILES_PER_CHUNK,
};

use bevy::prelude::*;
use itertools::Itertools;

pub mod draw_functions;

pub fn draw_chunks_from_instruction(
    mut commands: Commands,
    config: Res<CwConfig>,
    terrain_textures: Res<TerrainTextures>,
    chunk_instructions_query: Query<
        (Entity, &WorldChunk, &WorldChunkInstruction),
        Added<WorldChunkInstruction>,
    >,
) {
    for (chunk_id, chunk_world, chunk_instruction) in chunk_instructions_query.iter() {
        let chunk_debug_entity = match config.debug_flags.debug_chunk {
            true => Some(draw_debug_chunk_grid(
                &mut commands,
                &chunk_world.position,
                &terrain_textures.debug_grid,
            )),
            false => None,
        };

        let tile_entities: [Entity; TILES_PER_CHUNK] = chunk_instruction
            .tiles
            .iter()
            .map(|tile_to_spawn| {
                draw_single_tile(&mut commands, tile_to_spawn, &terrain_textures)
            })
            .collect_vec()
            .try_into()
            .unwrap();

        commands.entity(chunk_id).insert(TilesInChunk {
            chunk_debug_entity,
            tile_entities,
        });
    }
}
