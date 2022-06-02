pub mod biomes;
pub mod generator;
pub mod noise;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumVariantNames};

#[derive(EnumCount, EnumIter, EnumVariantNames, Display, Clone, Debug, Serialize, Deserialize)]
pub enum BaseTerrain {
    Stone = 0,
    Sand = 1,
    Grass = 2,
    Water = 3,
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

impl From<usize> for BaseTerrain {
    fn from(n: usize) -> Self {
        match n {
            0 => BaseTerrain::Stone,
            1 => BaseTerrain::Sand,
            2 => BaseTerrain::Grass,
            3 => BaseTerrain::Water,
            _ => panic!(),
        }
    }
}

pub type TerrainArray<T> = [T; BaseTerrain::COUNT];

pub const BORDER_ASSET_COUNT: usize = 20;
pub type TerrainBorderArray<T> = [[T; BORDER_ASSET_COUNT]; BaseTerrain::COUNT];

pub const TERRAIN_DECORATION_COUNT: usize = 20;
pub type TerrainDecorationArray<T> = [[T; TERRAIN_DECORATION_COUNT]; BaseTerrain::COUNT];
