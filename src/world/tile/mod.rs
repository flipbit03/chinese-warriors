use bevy::prelude::{Component, Transform};

use self::{border::structs::TileBorder, position::TilePosition, terrain::generator::Terrain};

pub mod border;
pub mod builder;
pub mod position;
pub mod terrain;
pub mod traits;
pub mod visibility;

#[derive(Component, Clone, Debug, Default)]
pub struct WorldTile {
    pub position: TilePosition,
    pub terrain: Terrain,
    pub borders: Vec<TileBorder>,
}

#[derive(Component, Debug)]
pub struct WorldTileDrawInstrucion {
    pub tile: WorldTile,
    pub transform: Transform,
}
