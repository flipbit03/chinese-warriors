pub mod biomes;
pub mod generator;
pub mod noise;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, VariantNames};

#[derive(
    EnumCount,
    EnumIter,
    VariantNames,
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
    FrozenWater = 7,
    Clay = 8,
    Gravel = 9,
    ThickStone = 10,
    Lava = 11,
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
            7 => BaseTerrain::FrozenWater,
            8 => BaseTerrain::Clay,
            9 => BaseTerrain::Gravel,
            10 => BaseTerrain::ThickStone,
            11 => BaseTerrain::Lava,

            x => {
                error!("Bateu o numero {} no BaseTerrain!\n", x);
                panic!()
            }
        }
    }
}

pub type ColorTriplet = (u8, u8, u8);

/// A Terrain Config Object used in the configs
///  strength is not available here as it comes from the array's position
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerrainConfig {
    pub name: String,
    pub base: BaseTerrain,
    pub color: Option<ColorTriplet>,
    pub decoration_color: Option<ColorTriplet>,
    pub walkable: bool,
    pub move_speed_multiplier: Option<f32>,
}

impl TerrainConfig {
    pub fn new(
        name: String,
        base: BaseTerrain,
        walkable: bool,
        move_speed_multiplier: Option<f32>,
    ) -> Self {
        Self {
            name,
            base,
            color: Default::default(),
            decoration_color: Default::default(),
            walkable,
            move_speed_multiplier,
        }
    }

    pub fn new_color(
        name: String,
        base: BaseTerrain,
        walkable: bool,
        color: Option<ColorTriplet>,
        decoration_color: Option<ColorTriplet>,
        move_speed_multiplier: Option<f32>,
    ) -> Self {
        Self {
            name,
            base,
            color,
            decoration_color,
            walkable,
            move_speed_multiplier,
        }
    }
}

// A Terrain used in the generator
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Terrain {
    pub name: String,
    pub strength: usize,
    pub base: BaseTerrain,
    pub color: Option<ColorTriplet>,
    pub decoration_color: Option<ColorTriplet>,
    pub walkable: bool,
    pub move_speed_multiplier: f32,
}

impl Terrain {
    pub fn terrain_sprite_color(&self) -> Color {
        match self.color {
            None => Color::WHITE,
            Some((r, g, b)) => Color::srgb_u8(r, g, b),
        }
    }

    pub fn decoration_sprite_color(&self) -> Color {
        match self.decoration_color {
            None => Color::WHITE,
            Some((r, g, b)) => Color::srgb_u8(r, g, b),
        }
    }

    pub fn new_from_config(c: &TerrainConfig, strength: usize) -> Self {
        Self {
            name: c.name.clone(),
            strength,
            base: c.base.clone(),
            color: c.color,
            decoration_color: c.decoration_color,
            walkable: c.walkable,
            move_speed_multiplier: c.move_speed_multiplier.unwrap_or(1.0),
        }
    }
}

/// How many border assets per BaseTerrain
pub const BORDER_ASSET_COUNT: usize = 20;

/// How many decorations per BaseTerrain
pub const TERRAIN_DECORATION_COUNT: usize = 20;
