use bevy::prelude::{Commands, OrthographicCameraBundle, Res};

use crate::systems::world::tile::TileBuilder;

use super::textures::TerrainTextures;

// SEED: cadu + jaw + kawe
pub static GAME_SEED: u32 = 7 + 18 + 9;

pub fn load_game_assets(mut commands: Commands, base_terrain_texture_atlas: Res<TerrainTextures>) {
    // Spawn Tile Builder with set game_seed;
    commands.insert_resource(TileBuilder::new_with_seed(
        GAME_SEED,
        base_terrain_texture_atlas.tile_size,
        1.0,
    ));

    // TODO: Implement Terrain Visibility Frustum
    //commands.insert_resource(WorldViewFrustum::default());

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
