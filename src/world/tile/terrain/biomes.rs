use std::{collections::HashMap, ops::Range};

use serde::{Deserialize, Serialize};

use super::{
    generator::TerrainName,
    noise::{NoiseGenerator, NoiseGeneratorSeedOffsetConfig},
    Terrain,
};

pub type BiomeName = String;

pub type Ranged<T> = (T, Range<f64>);

pub struct Biome {
    pub name: String,
    pub range: Range<f64>,
    pub terrains: Vec<Ranged<Terrain>>,
    pub terrain_count: usize,
    pub default_terrain: Terrain,
    pub noise_terrain: NoiseGenerator,
    pub noise_decoration: NoiseGenerator,
    pub decoration_eagerness: Option<Vec<Range<f64>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BiomeConfig {
    pub name: String,
    pub range: Range<f64>,
    pub terrains: Vec<Ranged<TerrainName>>,
    pub default_terrain: TerrainName,
    pub noise_terrain: NoiseGeneratorSeedOffsetConfig,
    pub noise_decoration: NoiseGeneratorSeedOffsetConfig,
    pub decoration_eagerness: Option<Vec<Range<f64>>>,
}

pub type BiomeDict<T> = HashMap<BiomeName, T>;
