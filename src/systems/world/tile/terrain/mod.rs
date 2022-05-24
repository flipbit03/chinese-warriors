pub mod generator;


use variant_count::VariantCount;

#[derive(VariantCount, Clone, Debug)]
pub enum Terrain {
    Sand = 0,
    Grass = 1,
}

impl Into<f32> for Terrain {
    fn into(self) -> f32 {
        (self as u16).try_into().unwrap()
    }
}

impl From<usize> for Terrain {
    fn from(n: usize) -> Self {
        match n {
            0 => Terrain::Sand,
            1 => Terrain::Grass,
            _ => panic!(),
        }
    }
}

pub static BASE_TERRAINS: [Terrain; Terrain::VARIANT_COUNT] = [Terrain::Sand, Terrain::Grass];

pub const TERRAIN_COUNT: usize = Terrain::VARIANT_COUNT;
pub const TERRAINS_WITH_BORDERS_COUNT: usize = Terrain::VARIANT_COUNT - 1;
pub const OUTER_BORDERS_COUNT: usize = 16;

pub type TerrainArray<T> = [T; TERRAIN_COUNT];
pub type TerrainOuterBorderArray<T> = [[T; OUTER_BORDERS_COUNT]; TERRAINS_WITH_BORDERS_COUNT];
