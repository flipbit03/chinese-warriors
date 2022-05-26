use std::collections::HashMap;

use bevy::{math::Vec2, prelude::{Transform, Component}};

use self::{
    border::TileBorder,
    position::TilePosition,
    terrain::{generator::{TerrainGenerator, Terrain}, BaseTerrain},
};

pub mod border;
pub mod builder;
pub mod position;
pub mod terrain;
pub mod traits;
pub mod visibility;

// enum TileBorder {
//     OuterTop = 0 ,
//     OuterRight = 1,
//     OuterBottom = 2,
//     OuterLeft = 3,
//     OuterCornerBottomLeft = 4,
//     OuterCornerBottomRight = 5,
//     OuterCornerBottomRight = 5
// }

#[derive(Component, Clone, Debug, Default)]
pub struct Tile {
    pub position: TilePosition,
    pub terrain: Terrain,
    pub borders: Vec<TileBorder>,
}

#[derive(Debug)]
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
