use bevy::{
    math::{Vec2, Vec3},
    prelude::Transform,
};
use serde::{Deserialize, Serialize};

use super::{
    border::structs::TileBorder,
    position::TilePositionNeighbors,
    terrain::generator::{TerrainGenerator, TerrainGeneratorConfig},
    TilePosition, WorldTile, WorldTileDrawInstrucion,
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

    fn get(&self, pos: TilePosition) -> WorldTile {
        let matrix = TilePositionNeighbors::new(&self.generator, pos);
        WorldTile {
            borders: TileBorder::from(&matrix, &self.generator),
            position: matrix.center.1,
            worldterrain: matrix.center.0,
        }
    }

    pub fn create(&self, tile_position: TilePosition) -> WorldTileDrawInstrucion {
        let tile = self.get(tile_position.clone());
        let terrain_z = tile.worldterrain.terrain.strength as f32 / 10000.0; // 0.0001
        WorldTileDrawInstrucion {
            terrain_sprite: tile.worldterrain.terrain.terrain_sprite_color(),
            decoration_sprite: tile.worldterrain.terrain.decoration_sprite_color(),
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
}
