use std::collections::HashSet;

use bevy::math::XY;
use bevy::prelude::Transform;

use bevy::prelude::*;
use bevy::utils::tracing::Id;
use noise::{NoiseFn, Perlin, Seedable, Fbm};

use self::frustum_culling::WorldViewFrustum;

use super::setup::TerrainTextureHandle;
use super::types::TilePosition;

pub mod frustum_culling;

#[derive(Component)]
pub struct Tile;

pub struct TerrainCreation {
    pub storage: HashSet<TilePosition>,
    pub perlin: Perlin,
    pub fbm: Fbm,
    pub perlin_scale_factor: XY<f64>
}

impl Default for TerrainCreation {
    fn default() -> Self {
        let seed = 7 + 18 + 9;

        let p = Perlin::new().set_seed(seed);

        let fbm = Fbm::new().set_seed(seed);

        Self {
            storage: Default::default(),
            perlin: p,
            fbm: fbm,
            perlin_scale_factor: XY {
                x: 96.0,
                y: 96.0
            }
        }
    }
}

impl TerrainCreation {
    pub fn get_tile_for_position(&self, tp: &TilePosition) -> f64 {
        let point = [tp.0 as f64 / self.perlin_scale_factor.x, tp.1 as f64 / self.perlin_scale_factor.y];
        (self.fbm.get(point) + self.perlin.get(point)) / 2.0
    }
}

pub fn spawn_terrain(
    mut commands: Commands,
    terrain_texture_handle: Res<TerrainTextureHandle>,
    world_frustum: Res<WorldViewFrustum>,
    mut terrain_creation: ResMut<TerrainCreation>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    let terrain_atlas_handle = texture_atlases.get_handle(terrain_texture_handle.terrain.clone());

    let visible_tiles: Vec<TilePosition> = world_frustum.get_visible_tiles();

    let mult = world_frustum.terrain_tile_size * world_frustum.terrain_scale_factor;

    for tile in visible_tiles {
        if !terrain_creation.storage.contains(&tile) {
            let new_tile_transform = Transform {
                translation: Vec3::new(
                    (tile.0 * mult as i32) as f32,
                    (tile.1 * mult as i32) as f32,
                    0.0,
                ),
                scale: Vec3::splat(world_frustum.terrain_scale_factor as f32),
                ..Default::default()
            };

            let perlin_output = terrain_creation.get_tile_for_position(&tile);

            let terrain_sprite_number = (((perlin_output + 1.0) / 2.0) * 11.0) as usize;

            println!("perlin({:?}) = {}f64", tile, perlin_output);

            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: terrain_sprite_number,
                        // color: Color::rgb(
                        //     (perlin_output + 1.0) as f32,
                        //     (perlin_output + 1.0) as f32,
                        //     (perlin_output + 1.0) as f32,
                        // ),
                        ..default()
                    },
                    texture_atlas: terrain_atlas_handle.clone(),
                    transform: new_tile_transform.clone(),
                    ..default()
                })
                .insert(Tile);
            terrain_creation.storage.insert(tile);
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
