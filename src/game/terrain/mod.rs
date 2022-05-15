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

    let visible_tiles: Vec<TilePosition> = world_frustum.get_visible_tiles();

    let mult = world_frustum.terrain_tile_size * world_frustum.terrain_scale_factor;

    for tile in visible_tiles {
        if !terrain_counter.storage.contains(&tile) {
            let new_tile_transform = Transform {
                translation: Vec3::new(
                    (tile.0 * mult as i32) as f32,
                    (tile.1 * mult as i32) as f32,
                    0.0,
                ),
                scale: Vec3::splat(world_frustum.terrain_scale_factor as f32),
                ..Default::default()
            };

            commands.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 24,
                    ..default()
                },
                texture_atlas: terrain_atlas_handle.clone(),
                transform: new_tile_transform.clone(),
                ..default()
            });
            terrain_counter.storage.insert(tile);
        }
    }
}
