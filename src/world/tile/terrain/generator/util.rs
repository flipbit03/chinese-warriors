use itertools::Itertools;

use crate::world::tile::terrain::{
    biomes::{Biome, BiomeDict},
    Terrain, TerrainConfig,
};

use super::TerrainName;

pub fn build_terrain_list(terrain_config_list: &Vec<TerrainConfig>) -> Vec<Terrain> {
    terrain_config_list
        .into_iter()
        .enumerate()
        .map(|(strength, config)| Terrain::new_from_config(config, strength))
        .collect()
}

pub fn build_biome_dict(
    terrain_list: &Vec<Terrain>,
    biome_config: &BiomeDict<TerrainName>,
) -> BiomeDict<Terrain> {
    let mut biomes = BiomeDict::<Terrain>::new();

    biome_config
        .into_iter()
        .for_each(|(biome_name, biome_config)| {
            biomes.insert(
                biome_name.to_owned(),
                Biome::<Terrain> {
                    range: biome_config.range.clone(),
                    terrains: (&biome_config.terrains)
                        .into_iter()
                        .map(|(terrain_name, range)| {
                            (
                                terrain_list
                                    .into_iter()
                                    .find(|t| t.name == *terrain_name)
                                    .unwrap()
                                    .clone(),
                                range.clone(),
                            )
                        })
                        .collect_vec(),
                    default_terrain: terrain_list
                        .into_iter()
                        .find(|t| t.name == biome_config.default_terrain)
                        .unwrap()
                        .clone(),
                },
            );
        });

    biomes
}
