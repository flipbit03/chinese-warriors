use std::collections::HashMap;

use bevy::math::Vec2;

use crate::{
    hero::structs::HeroConfig,
    utilities::xy::XY,
    world::tile::{
        builder::WorldBuilderConfig,
        terrain::{
            biomes::{Biome, BiomeDict},
            generator::{TerrainGeneratorConfig, TerrainName},
            noise::NoiseGeneratorConfig,
            BaseTerrain, TerrainConfig,
        },
    },
};

use super::structs::CwConfig;

impl Default for CwConfig {
    fn default() -> Self {
        let terrains: Vec<TerrainConfig> = [
            TerrainConfig::new("Stone".to_string(), BaseTerrain::Stone, true),
            TerrainConfig::new_color(
                "DarkStone".to_string(),
                BaseTerrain::Grass,
                true,
                (155, 155, 155),
            ),
            TerrainConfig::new("Sand".to_string(), BaseTerrain::Sand, true),
            TerrainConfig::new("Grass".to_string(), BaseTerrain::Grass, true),
            TerrainConfig::new_color(
                "DeepGrass".to_string(),
                BaseTerrain::Grass,
                true,
                (140, 255, 255),
            ),
            TerrainConfig::new_color(
                "TaintedGrass".to_string(),
                BaseTerrain::Grass,
                true,
                (140, 155, 79),
            ),
            TerrainConfig::new("ShallowWater".to_string(), BaseTerrain::ShallowWater, true),
            TerrainConfig::new("DeepWater".to_string(), BaseTerrain::DeepWater, false),
        ]
        .to_vec();

        let biomes: BiomeDict<TerrainName> = HashMap::from([(
            "FloodedRuins".to_string(),
            Biome::<TerrainName> {
                range: 0.0..1.0,
                terrains: [
                    ("DeepWater".to_string(), 0.1..0.2),
                    ("ShallowWater".to_string(), 0.2..0.5),
                    ("Stone".to_string(), 0.5..0.6),
                    ("Grass".to_string(), 0.6..0.7),
                    ("DeepGrass".to_string(), 0.7..0.85),
                    ("TaintedGrass".to_string(), 0.85..0.9),
                ]
                .to_vec(),
                default_terrain: "Stone".to_string(),
            },
        )]);

        Self {
            world: WorldBuilderConfig {
                terrain_generation: TerrainGeneratorConfig {
                    terrains,
                    biomes,
                    base_terrain: Default::default(),
                    decoration: NoiseGeneratorConfig {
                        noise_scale_factor: XY { x: 2.0, y: 2.0 },
                        ..Default::default()
                    },
                },
                tile_size: Vec2::new(64., 64.),
                tile_scale: 1.0,
            },
            hero: HeroConfig { move_speed: 2.1 },
        }
    }
}
