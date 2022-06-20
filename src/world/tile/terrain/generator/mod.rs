pub mod util;

use serde::{Deserialize, Serialize};

use crate::world::tile::{
    position::TilePosition,
    terrain::generator::util::{build_biome_list, build_initial_terrain_list},
};

use super::{
    biomes::{Biome, BiomeConfig},
    noise::{NoiseGenerator, NoiseGeneratorConfig, NoiseGeneratorSeedOffsetConfig},
    Terrain, TerrainConfig,
};

pub type TerrainName = String;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TerrainGeneratorConfig {
    pub seed: u32,
    pub terrains: Vec<TerrainConfig>,
    pub biomes: Vec<BiomeConfig>,
    pub biome_noise: NoiseGeneratorSeedOffsetConfig,
}

pub struct TerrainGenerator {
    pub terrains: Vec<Terrain>,
    pub terrain_count: usize,
    pub biomes: Vec<Biome>,
    pub biome_noise: NoiseGenerator,
}

#[derive(Debug, Clone)]
pub struct WorldTerrain {
    /// Terrain Material
    pub terrain: Terrain,

    // Decoration
    pub decoration: Option<usize>,
}

impl TerrainGenerator {
    pub fn new_from_config(config: &TerrainGeneratorConfig) -> Self {
        // Generate all initial Terrains from TerrainConfigs
        let initial_terrain_list = build_initial_terrain_list(&config.terrains);

        // Generate all Biomes, using the terrains above as templates.
        let biomes =
            build_biome_list(&initial_terrain_list, config.seed, &config.biomes);

        // Generate a global list of Terrains (and updated strengths) from the Biomes.
        let mut terrains = Vec::new();
        biomes.iter().for_each(|b| {
            b.terrains
                .iter()
                .for_each(|(t, _)| terrains.push(t.clone()));
        });
        let terrain_count = (&terrains).iter().count();

        Self {
            terrains,
            terrain_count,
            biomes,
            biome_noise: NoiseGenerator::new_from_config(
                &NoiseGeneratorConfig::from_seed_offset(
                    config.seed,
                    &config.biome_noise,
                ),
            ),
        }
    }

    fn get_biome_and_index(&self, tp: &TilePosition) -> &Biome {
        self.biomes
            .iter()
            .find(|biome| biome.range.contains(&self.biome_noise.get_noise(tp)))
            .unwrap_or(&self.biomes[0])
    }

    fn get_terrain(biome: &Biome, tp: &TilePosition) -> Terrain {
        biome
            .terrains
            .iter()
            .find(|(_, range)| range.contains(&biome.noise_terrain.get_noise(tp)))
            .unwrap_or(&(biome.default_terrain.clone(), 0.0..1.0))
            .0
            .clone()
    }

    pub const DECORATION_COUNT: usize = 20;

    fn get_decoration(biome: &Biome, tp: &TilePosition) -> Option<usize> {
        let decoration_noise = biome.noise_decoration.get_noise(tp);

        match (biome.decoration_eagerness.as_ref())
            .unwrap_or(&Vec::new())
            .iter()
            .find(|dr| dr.contains(&decoration_noise))
        {
            Some(found_range) => {
                // Normalize everyone from 0.0
                let max = found_range.end - found_range.start;
                let num = decoration_noise - found_range.start;

                let decoration =
                    (TerrainGenerator::DECORATION_COUNT as f64 * num) / max;

                Some(decoration as usize)
            }
            None => None,
        }
    }

    pub fn get_world_terrain(&self, tp: &TilePosition) -> WorldTerrain {
        let biome = self.get_biome_and_index(tp);

        let t = WorldTerrain {
            terrain: TerrainGenerator::get_terrain(&biome, &tp),
            decoration: TerrainGenerator::get_decoration(&biome, &tp),
        };
        t
    }
}
