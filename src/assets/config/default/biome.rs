use crate::world::tile::terrain::{biomes::BiomeConfig, noise::NoiseGeneratorSeedOffsetConfig};

pub fn generate_default_biome_set() -> Vec<BiomeConfig> {
    [
        BiomeConfig {
            name: "Beach".to_string(),
            range: 0.0..0.5,
            terrains: [
                ("Sand".to_string(), 0.0..0.7),
                ("DarkSand".to_string(), 0.7..1.0),
            ]
            .to_vec(),
            default_terrain: "Sand".to_string(),
            noise_terrain: NoiseGeneratorSeedOffsetConfig::default(),
            noise_decoration: NoiseGeneratorSeedOffsetConfig::new(0, 2.0, 2.0),
        },
        BiomeConfig {
            name: "FloodedRuins".to_string(),
            range: 0.5..1.0,
            terrains: [
                ("Stone".to_string(), 0.5..0.6),
                ("Grass".to_string(), 0.6..0.7),
                ("DeepGrass".to_string(), 0.7..0.85),
                ("TaintedGrass".to_string(), 0.85..0.9),
                ("ShallowWater".to_string(), 0.2..0.5),
                ("DeepWater".to_string(), 0.1..0.2),
            ]
            .to_vec(),
            default_terrain: "Stone".to_string(),
            noise_terrain: NoiseGeneratorSeedOffsetConfig::default(),
            noise_decoration: NoiseGeneratorSeedOffsetConfig::new(0, 2.0, 2.0),
        },
    ]
    .to_vec()
}
