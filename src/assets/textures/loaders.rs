use bevy::prelude::{AssetServer, Handle, Image};

use crate::world::tile::terrain::{BaseTerrain, BORDER_ASSET_COUNT, TERRAIN_DECORATION_COUNT};

pub struct TerrainHandles {
    pub base_terrain: BaseTerrain,
    pub base: Handle<Image>,
    pub decorations: Vec<Handle<Image>>,
    pub borders: Vec<Handle<Image>>,
}

pub fn load_terrain_assets(
    asset_server: &AssetServer,
    base_terrain: BaseTerrain,
) -> TerrainHandles {
    let base_folder_name: String =
        format!("art/terrain/{}", base_terrain.to_string().to_lowercase());

    // Base Tile
    let base: Handle<Image> = asset_server.load(format!("{}/tile.png", base_folder_name).as_str());

    // Decorations
    let decorations = (0..(TERRAIN_DECORATION_COUNT))
        .map(|n| {
            asset_server.load::<Image, &str>(
                &format!("{}/decoration/decoration{}.png", base_folder_name, n).as_str(),
            )
        })
        .collect::<Vec<Handle<Image>>>();

    // Borders
    let borders = (0..(BORDER_ASSET_COUNT))
        .map(|n| {
            asset_server.load::<Image, &str>(
                &format!("{}/border/border{}.png", base_folder_name, n).as_str(),
            )
        })
        .collect::<Vec<Handle<Image>>>();

    TerrainHandles {
        base_terrain,
        base,
        decorations,
        borders,
    }
}
