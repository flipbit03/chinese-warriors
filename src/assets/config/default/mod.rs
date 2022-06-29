use bevy::math::Vec2;

use crate::{
    hero::structs::HeroConfig,
    world::tile::{
        builder::WorldBuilderConfig,
        terrain::{
            generator::TerrainGeneratorConfig, noise::NoiseGeneratorSeedOffsetConfig,
        },
    },
};

use self::{biome::generate_default_biome_set, terrain::generate_default_terrain_set};

use super::structs::{CwConfig, CwDebugFlags};
pub mod biome;
pub mod terrain;

impl Default for CwConfig {
    fn default() -> Self {
        Self {
            debug_flags: CwDebugFlags {
                print_player_position: true,
                debug_chunk: true,
            },
            world: WorldBuilderConfig {
                terrain_generation: TerrainGeneratorConfig {
                    terrains: generate_default_terrain_set(false),
                    biomes: generate_default_biome_set(),
                    seed: 34,
                    biome_noise: NoiseGeneratorSeedOffsetConfig::new(0, 292.0, 292.0),
                },
                tile_size: Vec2::new(64., 64.),
                tile_scale: 1.0,
                debug_grid: false,
            },
            hero: HeroConfig {
                move_speed: 2.1,
                spawn_point: Vec2::new(0.0, 0.0),
            },
        }
    }
}

impl CwConfig {
    pub fn debug_config() -> Self {
        Self {
            debug_flags: CwDebugFlags {
                print_player_position: true,
                debug_chunk: true,
            },
            world: WorldBuilderConfig {
                terrain_generation: TerrainGeneratorConfig {
                    terrains: generate_default_terrain_set(true),
                    biomes: generate_default_biome_set(),
                    seed: 34,
                    biome_noise: NoiseGeneratorSeedOffsetConfig::new(0, 2.0, 2.0),
                },
                tile_size: Vec2::new(64., 64.),
                tile_scale: 1.0,
                debug_grid: true,
            },
            hero: HeroConfig {
                move_speed: 2.1,
                spawn_point: Vec2::new(15614.0, 8297.0),
            },
        }
    }
}
