use bevy::{math::Vec2, prelude::*, sprite::TextureAtlas};
use strum::IntoEnumIterator;

pub mod loaders;

use crate::world::tile::terrain::BaseTerrain;

use self::loaders::{load_terrain_assets, TerrainHandles};

pub struct GuriTextureAtlas {
    pub texture_handle: Handle<TextureAtlas>,
}

pub struct TerrainTextures {
    pub tile_size: Vec2,
    pub base_terrains: Vec<TerrainHandles>,
    pub debug_grid: Handle<Image>,
}

pub fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(TerrainTextures {
        tile_size: Vec2::new(64.0, 64.0),
        base_terrains: BaseTerrain::iter()
            .map(|base_terrain| {
                info!("Loading Terrain {:?}...", base_terrain);
                load_terrain_assets(&asset_server, base_terrain)
            })
            .collect(),
        debug_grid: asset_server.load("art/terrain/debug.png"),
    });
}
