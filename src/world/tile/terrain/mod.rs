pub mod biomes;
pub mod generator;
pub mod noise;
use bevy::prelude::Color;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumVariantNames};

#[derive(EnumCount, EnumIter, EnumVariantNames, Display, Clone, Debug, Serialize, Deserialize)]
pub enum BaseTerrain {
    Stone = 0,
    Sand = 1,
    Grass = 2,
    ShallowWater = 3,
    DeepWater = 4,
}

/// A Terrain Config Object used in the configs
///  strength is not available here as it comes from the array's position
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerrainConfig {
    pub name: String,
    pub base: BaseTerrain,
    pub color: Color,
    pub walkable: bool,
}

impl TerrainConfig {
    pub fn new(name: String, base: BaseTerrain, walkable: bool) -> Self {
        Self {
            name,
            base,
            color: Default::default(),
            walkable,
        }
    }
}

// A Terrain used in the generator
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Terrain {
    pub name: String,
    pub strength: usize,
    pub base: BaseTerrain,
    pub color: Color,
    pub walkable: bool,
}

impl Terrain {
    pub fn new_from_config(c: &TerrainConfig, strength: usize) -> Self {
        Self {
            name: c.name.clone(),
            strength,
            base: c.base.clone(),
            color: c.color,
            walkable: c.walkable,
        }
    }
}

impl From<usize> for BaseTerrain {
    fn from(n: usize) -> Self {
        match n {
            0 => BaseTerrain::Stone,
            1 => BaseTerrain::Sand,
            2 => BaseTerrain::Grass,
            3 => BaseTerrain::ShallowWater,
            4 => BaseTerrain::DeepWater,
            _ => panic!(),
        }
    }
}

impl Default for BaseTerrain {
    fn default() -> Self {
        Self::Sand
    }
}

impl Into<f32> for BaseTerrain {
    fn into(self) -> f32 {
        (self as u16).try_into().unwrap()
    }
}

pub type TerrainArray<T> = [T; BaseTerrain::COUNT];

pub const BORDER_ASSET_COUNT: usize = 20;
pub type TerrainBorderArray<T> = [[T; BORDER_ASSET_COUNT]; BaseTerrain::COUNT];

pub const TERRAIN_DECORATION_COUNT: usize = 20;
pub type TerrainDecorationArray<T> = [[T; TERRAIN_DECORATION_COUNT]; BaseTerrain::COUNT];
