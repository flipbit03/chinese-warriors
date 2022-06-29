use bevy::{
    math::{Vec2, Vec3},
    prelude::Transform,
};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::world::tile::chunk::WorldChunkBundle;

use super::{
    chunk::{WorldChunkInstruction, CHUNK_SIDE_SIZE},
    position::ChunkPosition,
    terrain::generator::{TerrainGenerator, TerrainGeneratorConfig},
    TilePosition, WorldTileDrawInstrucion,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WorldBuilderConfig {
    pub terrain_generation: TerrainGeneratorConfig,
    pub tile_size: Vec2,
    pub tile_scale: f32,
    pub debug_grid: bool,
}

pub struct WorldBuilder {
    pub tile_size: Vec2,
    pub tile_scale: f32,
    pub debug_grid: bool,
    generator: TerrainGenerator,
}

impl WorldBuilder {
    pub fn new_from_config(config: &WorldBuilderConfig) -> Self {
        WorldBuilder {
            generator: TerrainGenerator::new_from_config(&config.terrain_generation),
            tile_size: config.tile_size,
            tile_scale: config.tile_scale,
            debug_grid: config.debug_grid,
        }
    }

    pub fn create_tile_draw_instruction(
        &self,
        tile_position: &TilePosition,
    ) -> WorldTileDrawInstrucion {
        let tile = self.generator.get_world_tile(&tile_position);
        let terrain_z = tile.terrain.strength as f32 / 10000.0; // 0.0001
        WorldTileDrawInstrucion {
            tile,
            transform: Transform {
                translation: Vec3::new(
                    tile_position.x as f32 * self.tile_size.x * self.tile_scale,
                    tile_position.y as f32 * self.tile_size.y * self.tile_scale,
                    terrain_z,
                ),
                scale: Vec3::splat(self.tile_scale),
                ..Default::default()
            },
            debug_grid: self.debug_grid,
        }
    }

    pub fn get_chunk(&self, position: &ChunkPosition) -> WorldChunkBundle {
        let x_range = ((position.x * CHUNK_SIDE_SIZE) - (CHUNK_SIDE_SIZE / 2))
            ..=((position.x * CHUNK_SIDE_SIZE) + (CHUNK_SIDE_SIZE / 2));
        let y_range = ((position.y * CHUNK_SIDE_SIZE) - (CHUNK_SIDE_SIZE / 2))
            ..=((position.y * CHUNK_SIDE_SIZE) + (CHUNK_SIDE_SIZE / 2));

        let z = x_range.cartesian_product(y_range).collect_vec();

        WorldChunkBundle {
            transform: Transform::from_xyz(
                (position.x as f32
                    * CHUNK_SIDE_SIZE as f32
                    * self.tile_size.x
                    * self.tile_scale) as f32,
                (position.y as f32
                    * CHUNK_SIDE_SIZE as f32
                    * self.tile_size.y
                    * self.tile_scale) as f32,
                0.0,
            ),
            instruction: WorldChunkInstruction {
                tiles: z
                    .iter()
                    .map(|tile_pos| tile_pos.clone().into())
                    .map(|tile_pos: TilePosition| {
                        self.create_tile_draw_instruction(&tile_pos)
                    })
                    .collect_vec()
                    .try_into()
                    .unwrap(),
            },
        }
    }
}
