use std::{collections::HashMap, ops::Range};

use serde::{Deserialize, Serialize};

use crate::world::tile::position::TilePosition;

use super::{
    biomes::Biome,
    noise::{NoiseGenerator, NoiseGeneratorConfig},
    BaseTerrain,
};

pub type BiomeDict = HashMap<Biome, Vec<(BaseTerrain, Range<f64>)>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainGeneratorConfig {
    pub biomes: BiomeDict,
    pub base_terrain: NoiseGeneratorConfig,
    pub decoration: NoiseGeneratorConfig,
}

pub struct TerrainGenerator {
    pub biomes: BiomeDict,
    pub terrain_component: NoiseGenerator,
    pub decoration_component: NoiseGenerator,
}

#[derive(Debug, Default, Clone)]
pub struct Terrain {
    pub base: BaseTerrain,
    pub decoration: Option<usize>,
}

pub const DECORATION_COUNT: usize = 21;

impl TerrainGenerator {
    pub fn new_from_config(config: &TerrainGeneratorConfig) -> Self {
        Self {
            biomes: config.biomes.clone(),
            decoration_component: NoiseGenerator::new_from_config(&config.decoration),
            terrain_component: NoiseGenerator::new_from_config(&config.base_terrain),
        }
    }

    fn get_biome(&self) -> Biome {
        Biome::FloodedRuins
    }

    fn get_base_terrain(&self, tp: &TilePosition) -> BaseTerrain {
        let noise_value = self.terrain_component.get_noise(tp);

        let biome_terrain_proportions = self.biomes.get(&self.get_biome()).unwrap();

        for (terrain, range) in biome_terrain_proportions {
            if range.contains(&noise_value) {
                return terrain.clone();
            }
        }

        // Range was not found, default terrain is the lowest priority one.
        BaseTerrain::Stone
    }

    fn get_decoration(&self, tp: &TilePosition) -> Option<usize> {
        let noise_value = self.decoration_component.get_noise(tp);

        let decoration = (noise_value * ((DECORATION_COUNT - 1) as f64)) as usize;

        match decoration {
            0 => None,
            _ => Some(decoration - 1),
        }
    }

    pub fn get_terrain(&self, tp: &TilePosition) -> Terrain {
        let t = Terrain {
            base: self.get_base_terrain(&tp),
            decoration: self.get_decoration(&tp),
        };
        t
    }
}
