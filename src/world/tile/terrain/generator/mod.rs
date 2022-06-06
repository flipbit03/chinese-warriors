pub mod util;

use serde::{Deserialize, Serialize};

use crate::world::tile::{
    position::TilePosition,
    terrain::generator::util::{build_biome_dict, build_terrain_list},
};

use super::{
    biomes::{Biome, BiomeDict},
    noise::{NoiseGenerator, NoiseGeneratorConfig},
    Terrain, TerrainConfig,
};

pub type TerrainName = String;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TerrainGeneratorConfig {
    pub terrains: Vec<TerrainConfig>,
    pub biomes: BiomeDict<TerrainName>,
    pub base_terrain: NoiseGeneratorConfig,
    pub decoration: NoiseGeneratorConfig,
}

pub struct TerrainGenerator {
    pub terrains: Vec<Terrain>,
    pub terrain_count: usize,
    pub biomes: BiomeDict<Terrain>,
    pub terrain_component: NoiseGenerator,
    pub decoration_component: NoiseGenerator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldTerrain {
    pub terrain: Terrain,
    pub decoration: Option<usize>,
}

pub const DECORATION_COUNT: usize = 21;

impl TerrainGenerator {
    pub fn new_from_config(config: &TerrainGeneratorConfig) -> Self {
        // Generate all Terrains from TerrainConfigs
        let terrains = build_terrain_list(&config.terrains);
        let terrain_count = (&terrains).into_iter().count();

        // Generate all Biomes from Terrains
        let biomes = build_biome_dict(&terrains, &config.biomes);

        Self {
            terrains,
            terrain_count,
            biomes,
            decoration_component: NoiseGenerator::new_from_config(&config.decoration),
            terrain_component: NoiseGenerator::new_from_config(&config.base_terrain),
        }
    }

    fn get_biome(&self, _: &TilePosition) -> &Biome<Terrain> {
        self.biomes.values().next().unwrap()
    }

    fn get_terrain(&self, tp: &TilePosition) -> Terrain {
        let noise_value = self.terrain_component.get_noise(tp);

        let biome = self.get_biome(&tp);

        for (terrain, range) in &biome.terrains {
            if range.contains(&noise_value) {
                return terrain.clone();
            }
        }

        // Range was not found, default terrain is the lowest priority one.
        biome.default_terrain.clone()
    }

    fn get_decoration(&self, tp: &TilePosition) -> Option<usize> {
        let noise_value = self.decoration_component.get_noise(tp);

        let decoration = (noise_value * ((DECORATION_COUNT - 1) as f64)) as usize;

        match decoration {
            0 => None,
            _ => Some(decoration - 1),
        }
    }

    pub fn get_world_terrain(&self, tp: &TilePosition) -> WorldTerrain {
        let t = WorldTerrain {
            terrain: self.get_terrain(&tp),
            decoration: self.get_decoration(&tp),
        };
        t
    }
}
