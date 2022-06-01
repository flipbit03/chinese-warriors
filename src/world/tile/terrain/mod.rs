pub mod biomes;
pub mod generator;
pub mod noise;
use serde::{Deserialize, Serialize};
use variant_count::VariantCount;

#[derive(VariantCount, Clone, Debug, Serialize, Deserialize)]
pub enum BaseTerrain {
    Stone = 0,
    Sand = 1,
    Grass = 2,
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
            _ => panic!(),
        }
    }
}

pub const TERRAIN_COUNT: usize = BaseTerrain::VARIANT_COUNT;
pub type TerrainArray<T> = [T; TERRAIN_COUNT];

pub const TERRAINS_WITH_BORDERS_COUNT: usize = TERRAIN_COUNT - 1;
pub const BORDER_ASSET_COUNT: usize = 20;
pub type TerrainBorderArray<T> = [[T; BORDER_ASSET_COUNT]; TERRAINS_WITH_BORDERS_COUNT];

pub const TERRAIN_DECORATION_COUNT: usize = 20;
pub const TERRAINS_WITH_DECORATIONS_COUNT: usize = TERRAIN_COUNT;
pub type TerrainDecorationArray<T> =
    [[T; TERRAIN_DECORATION_COUNT]; TERRAINS_WITH_DECORATIONS_COUNT];
