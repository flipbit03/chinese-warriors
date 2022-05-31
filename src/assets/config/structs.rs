use bevy::{math::Vec2, reflect::TypeUuid};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, TypeUuid, Clone)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f8"]
pub struct CwConfig {
    pub terrain: TerrainConfig,
    pub hero: HeroConfig
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TerrainConfig {
    pub noise_seed: u32,
    pub tile_size: Vec2,
    pub tile_scale: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HeroConfig {
    pub move_speed: f32
}

impl Default for CwConfig {
    fn default() -> Self {
        Self {
            terrain: TerrainConfig {
                // SEED: cadu + jaw + kawe
                noise_seed: 7 + 18 + 9,
                tile_size: Vec2::new(64., 64.),
                tile_scale: 1.0,
            },
            hero: HeroConfig { move_speed: 2.1 }
        }
    }
}
