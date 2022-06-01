use std::collections::HashMap;

use bevy::math::Vec2;

use crate::{
    hero::structs::HeroConfig,
    utilities::xy::XY,
    world::tile::{
        builder::WorldBuilderConfig,
        terrain::{
            biomes::Biome,
            generator::{BiomeDict, TerrainGeneratorConfig},
            noise::NoiseGeneratorConfig,
            BaseTerrain,
        },
    },
};

use super::structs::CwConfig;

impl Default for CwConfig {
    fn default() -> Self {
        let biomes: BiomeDict = HashMap::from([(
            Biome::SandyForest,
            [
                (BaseTerrain::Stone, 0.0..0.2),
                (BaseTerrain::Sand, 0.2..0.3),
                (BaseTerrain::Grass, 0.5..1.0),
            ]
            .to_vec(),
        )]);

        Self {
            world: WorldBuilderConfig {
                terrain_generation: TerrainGeneratorConfig {
                    biomes,
                    base_terrain: NoiseGeneratorConfig {
                        seed: 7 + 18 + 9,
                        noise_scale_factor: XY { x: 4.0, y: 4.0 },
                    },
                    decoration: NoiseGeneratorConfig {
                        seed: 7 + 18 + 9,
                        noise_scale_factor: XY { x: 2.0, y: 2.0 },
                    },
                },
                tile_size: Vec2::new(64., 64.),
                tile_scale: 1.0,
            },
            hero: HeroConfig { move_speed: 2.1 },
        }
    }
}
