use bevy::{
    prelude::{Component, Transform},
    sprite::Sprite,
};

use self::{
    border::structs::TileBorder, position::TilePosition,
    terrain::generator::WorldTerrain,
};

pub mod border;
pub mod builder;
pub mod position;
pub mod terrain;
pub mod traits;
pub mod visibility;

#[derive(Component, Clone, Debug)]
pub struct WorldTile {
    pub position: TilePosition,
    pub worldterrain: WorldTerrain,
    pub borders: Vec<TileBorder>,
}

#[derive(Component, Debug)]
pub struct WorldTileDrawInstrucion {
    // Terrain Material + Borders + Decoration
    pub tile: WorldTile,

    // Global Position
    pub transform: Transform,

    // Draw a debug grid in this tile?
    pub debug_grid: bool,

    // Sprite config used to modify the terrain (mainly, the color attribute)
    pub terrain_sprite: Sprite,

    // Sprite config used to modify the decoration (mainly, the color attribute)
    pub decoration_sprite: Sprite,
}
