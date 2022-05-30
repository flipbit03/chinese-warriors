use bevy::{
    math::Vec2,
    prelude::{Component, Transform},
};

use self::{
    border::structs::TileBorder,
    position::TilePosition,
    terrain::generator::{Terrain, TerrainGenerator},
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

#[derive(Component, Debug)]
pub struct TileDrawInstrucion {
    pub tile: Tile,
    pub transform: Transform,
}

pub struct TileBuilder {
    pub tile_size: Vec2,
    pub tile_scale: f32,
    generator: TerrainGenerator,
}
