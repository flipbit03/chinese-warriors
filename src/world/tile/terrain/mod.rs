pub mod generator;

use variant_count::VariantCount;

#[derive(VariantCount, Clone, Debug)]
pub enum BaseTerrain {
    Sand = 0,
    Grass = 1,
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
            0 => BaseTerrain::Sand,
            1 => BaseTerrain::Grass,
            _ => panic!(),
        }
    }
}

pub static BASE_TERRAINS: [BaseTerrain; BaseTerrain::VARIANT_COUNT] =
    [BaseTerrain::Sand, BaseTerrain::Grass];

pub const TERRAIN_COUNT: usize = BaseTerrain::VARIANT_COUNT;
pub const TERRAINS_WITH_BORDERS_COUNT: usize = BaseTerrain::VARIANT_COUNT - 1;
pub const BORDER_ASSET_COUNT: usize = 20;

pub type TerrainArray<T> = [T; TERRAIN_COUNT];
pub type TerrainBorderArray<T> = [[T; BORDER_ASSET_COUNT]; TERRAINS_WITH_BORDERS_COUNT];

pub const TERRAIN_DECORATION_COUNT: usize = 20;
pub type TerrainDecorationArray<T> = [[T; TERRAIN_DECORATION_COUNT]; TERRAIN_COUNT];
