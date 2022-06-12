use bevy::math::Vec2;

use crate::{
    hero::structs::HeroConfig,
    world::tile::{
        builder::WorldBuilderConfig,
        terrain::{generator::TerrainGeneratorConfig, noise::NoiseGeneratorSeedOffsetConfig},
    },
};

use self::{biome::generate_default_biome_set, terrain::generate_default_terrain_set};

use super::structs::CwConfig;
pub mod biome;
pub mod terrain;

impl Default for CwConfig {
    fn default() -> Self {
        Self {
            world: WorldBuilderConfig {
                terrain_generation: TerrainGeneratorConfig {
                    terrains: generate_default_terrain_set(),
                    biomes: generate_default_biome_set(),
                    seed: 34,
                    biome_noise: NoiseGeneratorSeedOffsetConfig::new(0, 125.0, 125.0),
                },
                tile_size: Vec2::new(64., 64.),
                tile_scale: 1.0,
            },
            hero: HeroConfig {
                move_speed: 2.1,
                spawn_point: Vec2::new(15614.0, 8297.0),
            },
        }
    }
}
