use std::collections::HashMap;

use itertools::Itertools;

use crate::world::tile::terrain::{
    biomes::{Biome, BiomeDict},
    Terrain, TerrainConfig,
};

use super::TerrainName;

pub fn build_terrain_dict(terrains: &Vec<TerrainConfig>) -> HashMap<TerrainName, Terrain> {
    let mut terrains_dict: HashMap<TerrainName, Terrain> = HashMap::new();

    terrains
        .into_iter()
        .enumerate()
        .for_each(|(strength, config)| {
            terrains_dict.insert(
                config.name.clone(),
                Terrain::new_from_config(config, strength),
            );
        });

    terrains_dict
}

pub fn build_biome_dict(
    terrains_dict: &HashMap<TerrainName, Terrain>,
    biome_config: &BiomeDict<TerrainName>,
) -> BiomeDict<Terrain> {
    let mut biomes = BiomeDict::<Terrain>::new();

    biome_config.into_iter().for_each(|(biome, biome_config)| {
        biomes.insert(
            biome.to_owned(),
            Biome::<Terrain> {
                range: biome_config.range.clone(),
                terrains: (&biome_config.terrains)
                    .into_iter()
                    .map(|(terrain_name, range)| {
                        (
                            terrains_dict.get(terrain_name).unwrap().clone(),
                            range.clone(),
                        )
                    })
                    .collect_vec(),
                default_terrain: terrains_dict
                    .get(&biome_config.default_terrain)
                    .unwrap()
                    .clone(),
            },
        );
    });

    biomes
}
