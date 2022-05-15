use std::collections::HashSet;

use bevy::prelude::Transform;

use bevy::prelude::*;

use self::frustum_culling::WorldFrustum;

use super::setup::TerrainTextureHandle;
use super::types::TilePosition;

pub mod frustum_culling;

pub struct TerrainCreation {
    pub storage: HashSet<TilePosition>,
}

impl Default for TerrainCreation {
    fn default() -> Self {
        Self {
            storage: Default::default(),
        }
    }
}

pub fn spawn_terrain(
    mut commands: Commands,
    terrain_texture_handle: Res<TerrainTextureHandle>,
    world_frustum: Res<WorldFrustum>,
    mut terrain_counter: ResMut<TerrainCreation>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    let terrain_atlas_handle = texture_atlases.get_handle(terrain_texture_handle.terrain.clone());

    let visible_tiles = world_frustum.get_visible_tiles();

    for tile in visible_tiles {
        if !terrain_counter.storage.contains(&tile) {
            let mult = world_frustum.terrain_tile_size * world_frustum.terrain_scale_factor;

            let new_tile_transform = Transform {
                translation: Vec3::new(tile.0 as f32 * mult, tile.1 as f32 * mult, 0.0),
                scale: Vec3::splat(world_frustum.terrain_scale_factor),
                ..Default::default()
            };

            commands.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 25,
                    ..default()
                },
                texture_atlas: terrain_atlas_handle.clone(),
                transform: new_tile_transform,
                ..default()
            });

            terrain_counter.storage.insert(tile);
        }
    }
}

// fn spawn_tile(mut commands: Commands, position: TilePosition, tile_size: f32)
