use bevy::prelude::{Component, Transform};

use self::{border::structs::TileBorder, position::TilePosition, terrain::Terrain};

pub mod border;
pub mod builder;
pub mod chunk;
pub mod position;
pub mod terrain;
pub mod visibility;

#[derive(Component, Clone, Debug)]
pub struct WorldTile {
    pub position: TilePosition,
    pub terrain: Terrain,
    pub biome_name: String,
    pub decoration: Option<usize>,
    pub borders: Vec<TileBorder>,
}

#[derive(Component, Debug)]
pub struct WorldTileDrawInstrucion {
    pub tile: WorldTile,
    pub transform: Transform,
    pub debug_grid: bool,
}
