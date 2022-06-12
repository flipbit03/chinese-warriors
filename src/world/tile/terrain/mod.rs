pub mod biomes;
pub mod generator;
pub mod noise;
use bevy::{prelude::Color, sprite::Sprite};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumVariantNames};

#[derive(
    EnumCount,
    EnumIter,
    EnumVariantNames,
    Display,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub enum BaseTerrain {
    DeepWater = 0,
    Grass = 1,
    Stone = 2,
    Sand = 3,
    ShallowWater = 4,
    Snow = 5,
    Mud = 6,
}

impl From<usize> for BaseTerrain {
    fn from(n: usize) -> Self {
        match n {
            0 => BaseTerrain::DeepWater,
            1 => BaseTerrain::Grass,
            2 => BaseTerrain::Stone,
            3 => BaseTerrain::Sand,
            4 => BaseTerrain::ShallowWater,
            5 => BaseTerrain::Snow,
            6 => BaseTerrain::Mud,

            x => {
                println!("Bateu o numero {x} no BaseTerrain!\n");
                panic!()
            }
        }
    }
}

/// A Terrain Config Object used in the configs
///  strength is not available here as it comes from the array's position
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerrainConfig {
    pub name: String,
    pub base: BaseTerrain,
    pub color: Option<(u8, u8, u8)>,
    pub walkable: bool,
}

impl TerrainConfig {
    pub fn new(name: String, base: BaseTerrain, walkable: bool) -> Self {
        Self {
            name,
            base,
            color: Default::default(),
            walkable,
        }
    }

    pub fn new_color(name: String, base: BaseTerrain, walkable: bool, color: (u8, u8, u8)) -> Self {
        Self {
            name,
            base,
            color: Some(color),
            walkable,
        }
    }
}

// A Terrain used in the generator
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Terrain {
    pub name: String,
    pub strength: usize,
    pub base: BaseTerrain,
    pub color: Option<(u8, u8, u8)>,
    pub walkable: bool,
}

impl Terrain {
    pub fn sprite_color(&self) -> Sprite {
        if let None = self.color {
            return Sprite::default();
        };

        Sprite {
            color: {
                let (r, g, b) = self.color.unwrap();
                Color::rgb_u8(r, g, b)
            },
            ..Default::default()
        }
    }

    pub fn new_from_config(c: &TerrainConfig, strength: usize) -> Self {
        Self {
            name: c.name.clone(),
            strength,
            base: c.base.clone(),
            color: c.color.clone(),
            walkable: c.walkable,
        }
    }
}

/// How many border assets per BaseTerrain
pub const BORDER_ASSET_COUNT: usize = 20;

/// How many decorations per BaseTerrain
pub const TERRAIN_DECORATION_COUNT: usize = 20;
