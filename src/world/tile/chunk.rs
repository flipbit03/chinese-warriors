use bevy::prelude::{Bundle, Component, Entity, Transform};

use super::{position::ChunkPosition, WorldTileDrawInstrucion};

// How many tiles per dimension inside a chunk (x * x)
pub const CHUNK_SIDE_SIZE: i32 = 3;

// Total amount of tiles inside a Chunk
pub const TILES_PER_CHUNK: usize = (CHUNK_SIDE_SIZE * CHUNK_SIDE_SIZE) as usize;

#[derive(Component)]
pub struct WorldChunkInstruction {
    pub tiles: [WorldTileDrawInstrucion; TILES_PER_CHUNK],
}

#[derive(Component)]
pub struct WorldChunk {
    pub position: ChunkPosition,
}

#[derive(Bundle)]
pub struct WorldChunkBundle {
    pub transform: Transform,
    pub instruction: WorldChunkInstruction,
}

#[derive(Component)]
pub struct TilesInChunk {
    pub chunk_debug_entity: Option<Entity>,
    pub tile_entities: [Entity; TILES_PER_CHUNK],
}
