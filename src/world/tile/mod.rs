use bevy::prelude::{Component, Transform};

use self::{border::structs::TileBorder, position::TilePosition, terrain::Terrain};

pub mod border;
pub mod builder;
pub mod chunk;
pub mod position;
pub mod terrain;
pub mod traits;
pub mod visibility;

#[derive(Component, Clone, Debug)]
pub struct WorldTile {
    // Position in the world in 'tile' coordinate system
    pub position: TilePosition,

    /// Terrain Material
    pub terrain: Terrain,

    /// Biome Name
    pub biome_name: String,

    // Decoration
    pub decoration: Option<usize>,

    // Borders
    pub borders: Vec<TileBorder>,
}

#[derive(Component, Debug)]
pub struct WorldTileDrawInstrucion {
    // Terrain Material + Borders + Decoration
    pub tile: WorldTile,

    // True position in the world
    pub transform: Transform,

    // Draw a debug grid in this tile?
    pub debug_grid: bool,
}
