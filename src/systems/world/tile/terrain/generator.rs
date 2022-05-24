use bevy::math::XY;
use noise::{Fbm, NoiseFn, Perlin, Seedable};

use crate::systems::world::tile::position::TilePosition;

use super::{Terrain, BASE_TERRAINS};

pub struct TerrainGenerator {
    pub perlin: Perlin,
    pub fbm: Fbm,
    pub perlin_scale_factor: XY<f64>,
}

impl TerrainGenerator {
    pub fn new_with_seed(seed: u32) -> Self {
        let p = Perlin::new().set_seed(seed);
        let fbm = Fbm::new().set_seed(seed);
        Self {
            perlin: p,
            fbm: fbm,
            perlin_scale_factor: XY { x: 100., y: 100. },
        }
    }

    pub fn get_terrain(&self, tp: &TilePosition) -> Terrain {
        let point = [
            tp.x as f64 / self.perlin_scale_factor.x,
            tp.y as f64 / self.perlin_scale_factor.y,
        ];

        // 2 noise components in the [-0.5, 0.5] range
        let fbm_value = self.fbm.get(point) / 2.0;
        let perlin_value = self.perlin.get(point) / 2.0;

        // Sum both, get a value "somewhere" in the [-1.0, 1.0] range
        let combined = fbm_value + perlin_value; // -1 a 1

        // Add 1.0 (range becomes [0.0, 2.0], divide by 2, final normalized range becomes [0.0, 1.0]
        let normalized = ((combined + 1.0) / 2.0).clamp(0.0, 1.0); // [0,1]

        // Get final tile number
        let base_terrain_number = (normalized * Terrain::VARIANT_COUNT as f64) as usize;

        BASE_TERRAINS[base_terrain_number].clone()
    }
}
