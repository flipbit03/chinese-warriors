use crate::world::tile::terrain::{
    biomes::BiomeConfig, noise::NoiseGeneratorSeedOffsetConfig,
};

pub fn generate_default_biome_set() -> Vec<BiomeConfig> {
    [
        BiomeConfig {
            name: "RockDesert".to_string(),
            range: 0.0..0.2,
            terrains: [
                ("Sand".to_string(), 0.7..1.0),
                ("SandStone".to_string(), 0.5..0.7),
                ("RedStone".to_string(), 0.0..0.5),
            ]
            .to_vec(),
            default_terrain: "Sand".to_string(),
            noise_terrain: NoiseGeneratorSeedOffsetConfig::new(0, 5.0, 5.0),
            noise_decoration: NoiseGeneratorSeedOffsetConfig::new(0, 2.0, 2.0),
            decoration_eagerness: Some(vec![0.0..0.45, 0.75..1.00]),
        },
        BiomeConfig {
            name: "Quarry".to_string(),
            range: 0.3..0.35,
            terrains: [
                ("Sand".to_string(), 0.7..1.0),
                ("DarkStone".to_string(), 0.5..0.7),
                ("Stone".to_string(), 0.2..0.5),
                ("RedStone".to_string(), 0.0..0.2),
            ]
            .to_vec(),
            default_terrain: "Stone".to_string(),
            noise_terrain: NoiseGeneratorSeedOffsetConfig::new(0, 5.0, 5.0),
            noise_decoration: NoiseGeneratorSeedOffsetConfig::new(0, 2.0, 2.0),
            decoration_eagerness: Some(vec![0.0..0.45, 0.75..1.00]),
        },
        BiomeConfig {
            name: "Desert".to_string(),
            range: 0.35..0.5,
            terrains: [
                ("SandStone".to_string(), 0.0..0.2),
                ("SandStone".to_string(), 0.85..1.0),
                ("Sand".to_string(), 0.2..0.85),
            ]
            .to_vec(),
            default_terrain: "Sand".to_string(),
            noise_terrain: NoiseGeneratorSeedOffsetConfig::new(0, 20.0, 20.0),
            noise_decoration: NoiseGeneratorSeedOffsetConfig::new(0, 2.0, 2.0),
            decoration_eagerness: Some(vec![0.0..0.45, 0.75..1.00]),
        },
        BiomeConfig {
            name: "Oasis".to_string(),
            range: 0.25..0.3,
            terrains: [
                ("Sand".to_string(), 0.0..0.25),
                ("WetSand".to_string(), 0.25..0.3),
                ("WetSand".to_string(), 0.7..0.8),
                ("Sand".to_string(), 0.8..1.0),
                ("ShallowWater".to_string(), 0.3..0.7),
            ]
            .to_vec(),
            default_terrain: "Sand".to_string(),
            noise_terrain: NoiseGeneratorSeedOffsetConfig::new(0, 15.0, 15.0),
            noise_decoration: NoiseGeneratorSeedOffsetConfig::new(0, 2.0, 2.0),
            decoration_eagerness: Some(vec![0.0..0.45, 0.75..1.00]),
        },
        BiomeConfig {
            name: "Plains".to_string(),
            range: 0.6..0.78,
            terrains: [
                ("ShallowWater".to_string(), 0.0..0.1),
                ("Clay".to_string(), 0.1..0.24),
                ("Grass".to_string(), 0.24..0.55),
                ("Grass".to_string(), 0.8..1.0),
                ("PlainsGrass".to_string(), 0.55..0.8),
            ]
            .to_vec(),
            default_terrain: "Grass".to_string(),
            noise_terrain: NoiseGeneratorSeedOffsetConfig::new(0, 10.0, 10.0),
            noise_decoration: NoiseGeneratorSeedOffsetConfig::new(0, 2.0, 2.0),
            decoration_eagerness: Some(vec![0.0..0.45, 0.75..1.00]),
        },
        BiomeConfig {
            name: "Swamp".to_string(),
            range: 0.5..0.6,
            terrains: [
                ("DarkMud".to_string(), 0.5..0.6),
                ("DarkGrass".to_string(), 0.6..0.7),
                ("SwampGrass".to_string(), 0.7..1.0),
                ("SwampWater".to_string(), 0.25..0.6),
            ]
            .to_vec(),
            default_terrain: "DarkGrass".to_string(),
            noise_terrain: NoiseGeneratorSeedOffsetConfig::new(0, 4.0, 4.0),
            noise_decoration: NoiseGeneratorSeedOffsetConfig::new(0, 2.0, 2.0),
            decoration_eagerness: Some(vec![0.0..0.45, 0.75..1.00]),
        },
        BiomeConfig {
            name: "Tundra".to_string(),
            range: 0.5..0.6,
            terrains: [
                ("TundraGrass".to_string(), 0.0..0.5),
                ("FrozenGrass".to_string(), 0.5..1.0),
            ]
            .to_vec(),
            default_terrain: "TundraGrass".to_string(),
            noise_terrain: NoiseGeneratorSeedOffsetConfig::new(0, 9.0, 9.0),
            noise_decoration: NoiseGeneratorSeedOffsetConfig::new(0, 2.0, 2.0),
            decoration_eagerness: Some(vec![0.0..0.45, 0.75..1.00]),
        },
        BiomeConfig {
            name: "Snow".to_string(),
            range: 0.83..1.0,
            terrains: [
                ("ShallowWater".to_string(), 0.0..0.35),
                ("BlueSnow".to_string(), 0.9..1.0),
                ("BlueSnow".to_string(), 0.35..0.45),
                ("Snow".to_string(), 0.45..0.9),
            ]
            .to_vec(),
            default_terrain: "Snow".to_string(),
            noise_terrain: NoiseGeneratorSeedOffsetConfig::new(0, 25.0, 25.0),
            noise_decoration: NoiseGeneratorSeedOffsetConfig::new(0, 2.0, 2.0),
            decoration_eagerness: Some(vec![0.0..0.45, 0.75..1.00]),
        },
    ]
    .to_vec()
}
