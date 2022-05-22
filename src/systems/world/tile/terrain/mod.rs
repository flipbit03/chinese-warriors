pub mod generator;

use variant_count::VariantCount;

#[derive(VariantCount, Clone, Debug)]
pub enum Terrain {
    Sand = 0,
    Grass = 1,
}

pub static BASE_TERRAINS: [Terrain; Terrain::VARIANT_COUNT] = [Terrain::Sand, Terrain::Grass];
