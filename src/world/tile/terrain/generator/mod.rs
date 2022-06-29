pub mod util;

use serde::{Deserialize, Serialize};

use crate::world::tile::{
    border::structs::TileBorder,
    position::{TilePosition, TilePositionNeighbors},
    terrain::generator::util::{build_biome_list, build_initial_terrain_list},
    WorldTile,
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

impl TerrainGenerator {
    pub fn new_from_config(config: &TerrainGeneratorConfig) -> Self {
        // Generate all initial Terrains from TerrainConfigs
        let initial_terrain_list = build_initial_terrain_list(&config.terrains);

        // Generate all Biomes, using the terrains above as templates.
        let biomes = build_biome_list(&initial_terrain_list, config.seed, &config.biomes);

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
                &NoiseGeneratorConfig::from_seed_offset(config.seed, &config.biome_noise),
            ),
        }
    }

    pub fn get_biome(&self, tp: &TilePosition) -> &Biome {
        self.biomes
            .iter()
            .find(|biome| biome.range.contains(&self.biome_noise.get_noise(tp)))
            .unwrap_or(&self.biomes[0])
    }

    pub fn get_terrain(&self, tp: &TilePosition) -> Terrain {
        let biome = self.get_biome(&tp);
        biome
            .terrains
            .iter()
            .find(|(_, range)| range.contains(&biome.noise_terrain.get_noise(tp)))
            .unwrap_or(&(biome.default_terrain.clone(), 0.0..1.0))
            .0
            .clone()
    }

    pub const DECORATION_COUNT: usize = 20;

    fn get_decoration(&self, tp: &TilePosition) -> Option<usize> {
        let biome = self.get_biome(&tp);
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

                let decoration = (TerrainGenerator::DECORATION_COUNT as f64 * num) / max;

                Some(decoration as usize)
            }
            None => None,
        }
    }

    pub fn get_world_tile(&self, tp: &TilePosition) -> WorldTile {
        let biome = self.get_biome(tp);

        let matrix = TilePositionNeighbors::new(&self, tp.clone());

        let borders = TileBorder::from(&matrix, &self.terrains, self.terrain_count);

        let t = WorldTile {
            borders,
            position: matrix.center.1,
            biome_name: biome.name.clone(),
            terrain: self.get_terrain(&tp),
            decoration: self.get_decoration(&tp),
        };
        t
    }
}
