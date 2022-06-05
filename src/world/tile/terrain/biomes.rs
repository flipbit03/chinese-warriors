use std::{collections::HashMap, ops::Range};

use serde::{Deserialize, Serialize};

pub type BiomeName = String;

pub type Ranged<T> = (T, Range<f64>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Biome<T> {
    pub range: Range<f64>,
    pub terrains: Vec<Ranged<T>>,
    pub default_terrain: T,
}

pub type BiomeDict<T> = HashMap<BiomeName, Biome<T>>;
