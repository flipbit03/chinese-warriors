use bevy::{
    math::Vec2,
    prelude::{AssetServer, Assets, Commands, Handle, Res, ResMut},
    sprite::TextureAtlas,
};
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
}

pub fn load_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.insert_resource(TerrainTextures {
        tile_size: Vec2::new(64.0, 64.0),
        base_terrains: BaseTerrain::iter()
            .map(|base_terrain| {
                println!("Loading Terrain {:?}...", base_terrain);
                load_terrain_assets(&asset_server, base_terrain)
            })
            .collect(),
    });

    commands.insert_resource(GuriTextureAtlas {
        texture_handle: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("art/hero/guri/spritesheet.png"),
            Vec2::new(32.0, 32.0),
            7,
            1,
        )),
    });
}
