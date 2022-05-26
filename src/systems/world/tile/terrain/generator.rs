use bevy::math::XY;
use noise::{Fbm, NoiseFn, Perlin, Seedable};

use crate::systems::world::tile::position::TilePosition;

use super::{BaseTerrain, BASE_TERRAINS};

pub struct TerrainGenerator {
    pub perlin: Perlin,
    pub fbm: Fbm,
    pub terrain_noise_scale_factor: XY<f64>,
    pub decoration_noise_scale_factor: XY<f64>,
}

#[derive(Debug, Default, Clone)]
pub struct Terrain {
    pub base: BaseTerrain,
    pub decoration: Option<usize>
}

pub const DECORATION_COUNT: usize = 21;

impl TerrainGenerator {
    pub fn new_with_seed(seed: u32) -> Self {
        let p = Perlin::new().set_seed(seed);
        let fbm = Fbm::new().set_seed(seed);
        Self {
            perlin: p,
            fbm: fbm,
            terrain_noise_scale_factor: XY { x: 100., y: 100. },
            decoration_noise_scale_factor: XY { x: 5., y: 5. },
        }
    }

    fn get_base_terrain(&self, tp: &TilePosition) -> BaseTerrain {

        let terrain_point = [
            tp.x as f64 / self.terrain_noise_scale_factor.x,
            tp.y as f64 / self.terrain_noise_scale_factor.y,
        ];

        // 2 noise components in the [-0.5, 0.5] range
        let fbm_value = self.fbm.get(terrain_point) / 2.0;
        let perlin_value = self.perlin.get(terrain_point) / 2.0;

        // Sum both, get a value "somewhere" in the [-1.0, 1.0] range
        let combined = fbm_value + perlin_value; // -1 a 1

        // Add 1.0 (range becomes [0.0, 2.0], divide by 2, final normalized range becomes [0.0, 0.9999999999999999]
        let normalized = ((combined + 1.0) / 2.0).clamp(0.0, 1.0-1e-16);

        // from the normalized number, get the tile number
        let base_terrain_number = (normalized * (BaseTerrain::VARIANT_COUNT) as f64) as usize;
        
        // Get final tile number
        BASE_TERRAINS[base_terrain_number].clone()
    }

    fn get_decoration(&self, tp: &TilePosition) -> Option<usize> {

        let decoration_point = [
            tp.x as f64 / self.decoration_noise_scale_factor.x,
            tp.y as f64 / self.decoration_noise_scale_factor.y,
        ];

        // 2 noise components in the [-0.5, 0.5] range
        let fbm_value = self.fbm.get(decoration_point) / 2.0;
        let perlin_value = self.perlin.get(decoration_point) / 2.0;

        // Sum both, get a value "somewhere" in the [-1.0, 1.0] range
        let combined = fbm_value + perlin_value; // -1 a 1

        // Add 1.0 (range becomes [0.0, 2.0], divide by 2, final normalized range becomes [0.0, 1.0]
        let normalized = ((combined + 1.0) / 2.0).clamp(0.0, 1.0); // [0,1]

        let decoration = (normalized  * (DECORATION_COUNT as f64)) as usize;

        match decoration {
            0 => None,
            _ => Some(decoration - 1)
        }
    }

    pub fn get_terrain(&self, tp: &TilePosition) -> Terrain {
        let t = Terrain {
            base: self.get_base_terrain(&tp),
            decoration: self.get_decoration(&tp)
        };
        println!("Terrain => {:?}", &t);
        t
    }
}
