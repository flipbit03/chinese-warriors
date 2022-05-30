use bevy::{
    math::Vec2,
    prelude::{AssetServer, Assets, Commands, Handle, Image, Res, ResMut},
    sprite::TextureAtlas,
};

use crate::world::tile::terrain::{
    TerrainArray, TerrainBorderArray, TerrainDecorationArray, BORDER_ASSET_COUNT,
    TERRAIN_DECORATION_COUNT,
};

pub struct GuriTextureAtlas {
    pub texture_handle: Handle<TextureAtlas>,
}

pub struct TerrainTextures {
    pub tile_size: Vec2,
    pub base_terrain: TerrainArray<Handle<Image>>,
    pub borders: TerrainBorderArray<Handle<Image>>,
    pub decorations: TerrainDecorationArray<Handle<Image>>,
}

pub fn load_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    //
    // Stone
    //

    // Base Tile
    let bt_stone_handle: Handle<Image> = asset_server.load("Terrain/Stone/Stone.png");

    // Decorations
    let stone_decorations: Vec<Handle<Image>> = (0..(TERRAIN_DECORATION_COUNT))
        .map(|n| {
            let border_fn = format!("Terrain/Stone/Decoration/TerrainDecoration{n}.png");
            let loaded_image: Handle<Image> = asset_server.load(&border_fn);
            loaded_image
        })
        .collect();

    //
    // Sand
    //

    // Base Tile
    let bt_sand_handle: Handle<Image> = asset_server.load("Terrain/Sand/Sand.png");

    // Decorations
    let sand_decorations: Vec<Handle<Image>> = (0..(TERRAIN_DECORATION_COUNT))
        .map(|n| {
            let border_fn = format!("Terrain/Sand/Decoration/TerrainDecoration{n}.png");
            let loaded_image: Handle<Image> = asset_server.load(&border_fn);
            loaded_image
        })
        .collect();

    let sand_borders: Vec<Handle<Image>> = (0..(BORDER_ASSET_COUNT))
        .map(|n| {
            let border_fn = format!("Terrain/Sand/Border/SandBorder{n}.png");
            let loaded_image: Handle<Image> = asset_server.load(&border_fn);
            loaded_image
        })
        .collect();

    //
    // Sand
    //

    // Base Tile
    let bt_grass_handle: Handle<Image> = asset_server.load("Terrain/Grass/Grass.png");

    // Decorations
    let grass_decorations: Vec<Handle<Image>> = (0..(TERRAIN_DECORATION_COUNT))
        .map(|n| {
            let border_fn = format!("Terrain/Grass/Decoration/TerrainDecoration{n}.png");
            let loaded_image: Handle<Image> = asset_server.load(&border_fn);
            loaded_image
        })
        .collect();

    // Borders
    let grass_borders: Vec<Handle<Image>> = (0..(BORDER_ASSET_COUNT))
        .map(|n| {
            let border_fn = format!("Terrain/Grass/Border/GrassBorder{n}.png");
            let loaded_image: Handle<Image> = asset_server.load(&border_fn);
            loaded_image
        })
        .collect();

    let bt_textures = TerrainTextures {
        tile_size: Vec2::new(64.0, 64.0),
        base_terrain: [bt_stone_handle, bt_sand_handle, bt_grass_handle],
        borders: [
            sand_borders.try_into().unwrap(),
            grass_borders.try_into().unwrap(),
        ],
        decorations: [
            stone_decorations.try_into().unwrap(),
            sand_decorations.try_into().unwrap(),
            grass_decorations.try_into().unwrap(),
        ],
    };

    commands.insert_resource(bt_textures);

    commands.insert_resource(GuriTextureAtlas {
        texture_handle: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("Guri.png"),
            Vec2::new(32.0, 32.0),
            7,
            1,
        )),
    });
}
