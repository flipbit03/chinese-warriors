use itertools::Itertools;

use crate::world::tile::terrain::{
    biomes::{Biome, BiomeConfig},
    noise::{NoiseGenerator, NoiseGeneratorConfig},
    Terrain, TerrainConfig,
};

pub fn build_initial_terrain_list(terrain_config_list: &Vec<TerrainConfig>) -> Vec<Terrain> {
    terrain_config_list
        .into_iter()
        .map(|config| Terrain::new_from_config(config, 0))
        .collect()
}

pub fn build_biome_list(
    initial_terrain_list: &Vec<Terrain>,
    seed: u32,
    biome_config: &Vec<BiomeConfig>,
) -> Vec<Biome> {
    biome_config
        .into_iter()
        .enumerate()
        .map(|(biome_order, biome_config)| {
            let customized_terrain_list = (&biome_config.terrains)
                .into_iter()
                .enumerate()
                .map(|(biome_terrain_order, (terrain_name, terrain_range))| {
                    (
                        Terrain {
                            strength: biome_order * 10 + biome_terrain_order,
                            ..initial_terrain_list
                                .into_iter()
                                .find(|t| t.name == *terrain_name)
                                .unwrap()
                                .clone()
                        },
                        terrain_range.clone(),
                    )
                })
                .collect_vec();

            let customized_terrain_list_count = customized_terrain_list.len();

            Biome {
                name: biome_config.name.clone(),
                range: biome_config.range.clone(),
                default_terrain: customized_terrain_list
                    .iter()
                    .find(|(t, _)| t.name == biome_config.default_terrain)
                    .unwrap()
                    .0
                    .clone(),
                terrains: customized_terrain_list,
                terrain_count: customized_terrain_list_count,
                noise_terrain: NoiseGenerator::new_from_config(
                    &NoiseGeneratorConfig::from_seed_offset(seed, &biome_config.noise_terrain),
                ),
                noise_decoration: NoiseGenerator::new_from_config(
                    &NoiseGeneratorConfig::from_seed_offset(seed, &biome_config.noise_decoration),
                ),
                decoration_eagerness: biome_config.decoration_eagerness.clone(),
            }
        })
        .collect_vec()
}
