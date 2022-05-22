use std::collections::HashMap;

use bevy::{math::Vec2, prelude::Transform};

use self::{
    position::TilePosition,
    terrain::{generator::TerrainGenerator, Terrain},
};

pub mod builder;
pub mod position;
pub mod terrain;
pub mod traits;
pub mod visibility;

#[derive(Clone)]
pub struct Tile {
    pub terrain: Terrain,
}

pub struct TileDrawInstrucion {
    pub tile: Tile,
    pub transform: Transform,
}

pub struct TileBuilder {
    pub tile_size: Vec2,
    pub tile_scale: f32,
    pub storage: HashMap<TilePosition, Tile>,
    generator: TerrainGenerator,
}
