use std::collections::HashSet;

use bevy::prelude::Transform;

use bevy::prelude::*;
use bevy::utils::tracing::Id;

use self::frustum_culling::WorldViewFrustum;

use super::setup::TerrainTextureHandle;
use super::types::TilePosition;

pub mod frustum_culling;

#[derive(Component)]
pub struct Tile;

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
    world_frustum: Res<WorldViewFrustum>,
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

            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: 24,
                        ..default()
                    },
                    texture_atlas: terrain_atlas_handle.clone(),
                    transform: new_tile_transform.clone(),
                    ..default()
                })
                .insert(Tile);
            terrain_counter.storage.insert(tile);
        }
    }
}

pub fn despawn_terrain(
    mut commands: Commands,
    world_frustum: Res<WorldViewFrustum>,
    mut terrain_counter: ResMut<TerrainCreation>,
    mut tile_query: Query<(Entity, &Transform), With<Tile>>,
) {
    let tile_mult = (world_frustum.terrain_tile_size * world_frustum.terrain_scale_factor) as f32;

    for (entity, entity_transform) in tile_query.iter() {
        if !world_frustum.is_viewing(&entity_transform.translation) {
            let entity_tile_pos = (
                (entity_transform.translation.x / tile_mult) as i32,
                (entity_transform.translation.y / tile_mult) as i32,
            );
            if terrain_counter.storage.remove(&entity_tile_pos) {
                //println!("Limpei entidade {:?}", &entity);
                commands.entity(entity).despawn();
            }
        }
    }
}
