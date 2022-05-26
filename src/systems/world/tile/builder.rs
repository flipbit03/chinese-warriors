use bevy::{
    math::{Vec2, Vec3},
    prelude::Transform,
};

use super::{
    border::TileBorder, position::TilePositionNeighbors, terrain::generator::TerrainGenerator,
    Tile, TileBuilder, TileDrawInstrucion, TilePosition,
};

impl TileBuilder {
    pub fn new_with_seed(seed: u32, tile_size: Vec2, tile_scale: f32) -> Self {
        TileBuilder {
            generator: TerrainGenerator::new_with_seed(seed),
            storage: Default::default(),
            tile_size,
            tile_scale,
        }
    }

    fn get(&self, pos: TilePosition) -> Tile {
        let matrix = TilePositionNeighbors::new(&self.generator, pos);
        Tile {
            position: matrix.center.1.clone(),
            terrain: matrix.center.0.clone(),
            borders: TileBorder::from(matrix),
        }
    }

    pub fn create(&mut self, tile_position: TilePosition) -> TileDrawInstrucion {
        let tile = self.get(tile_position.clone());
        self.storage.insert(tile_position.clone(), tile.clone());
        let tile_z_order: f32 = tile.terrain.base.clone().into();
        TileDrawInstrucion {
            tile,
            transform: Transform {
                translation: Vec3::new(
                    tile_position.x as f32 * self.tile_size.x * self.tile_scale,
                    tile_position.y as f32 * self.tile_size.y * self.tile_scale,
                    tile_z_order/ 100.0,
                ),
                scale: Vec3::splat(self.tile_scale),
                ..Default::default()
            },
        }
    }
}
