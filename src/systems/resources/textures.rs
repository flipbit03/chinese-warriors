use bevy::{
    math::Vec2,
    prelude::{AssetServer, Assets, Commands, Handle, Res, ResMut},
    sprite::TextureAtlas,
};

pub struct GuriTextureAtlas {
    pub texture_handle: Handle<TextureAtlas>,
}

pub struct BaseTerrainTextureAtlas {
    pub tile_size: Vec2,
    pub terrain: Handle<TextureAtlas>,
}

pub fn load_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let base_terrain_tile_size = Vec2::new(64.0, 64.0);
    commands.insert_resource(BaseTerrainTextureAtlas {
        terrain: texture_atlases.add(TextureAtlas::from_grid_with_padding(
            asset_server.load("BaseTerrain.png"),
            base_terrain_tile_size,
            2,
            1,
            Vec2::new(0.0, 0.0),
        )),
        tile_size: base_terrain_tile_size,
    });

    commands.insert_resource(GuriTextureAtlas {
        texture_handle: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("Guri.png"),
            Vec2::new(32.0, 32.0),
            7,
            1,
        )),
    });
}
