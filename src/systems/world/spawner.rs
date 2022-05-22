use super::tile::traits::WidthHeight;
use super::tile::TileBuilder;
use crate::systems::resources::textures::BaseTerrainTextureAtlas;
use crate::systems::world::tile::visibility::{get_screen_rect, get_visible_tiles};
use bevy::prelude::Transform;
use bevy::prelude::*;
use bevy::render::camera::Camera2d;

#[derive(Component)]
pub struct Tile;

pub fn spawn_terrain(
    mut commands: Commands,
    terrain_texture_handle: Res<BaseTerrainTextureAtlas>,
    camera_query: Query<(&Transform, &OrthographicProjection), With<Camera2d>>,
    mut tile_builder: ResMut<TileBuilder>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    let terrain_atlas_handle = texture_atlases.get_handle(terrain_texture_handle.terrain.clone());

    let (camera_transform, camera_projection) = camera_query.single();

    let screen_rect = get_screen_rect(camera_transform, camera_projection, 0.2);

    // println!(
    //     "t= {:?} || p= {:?} | r= {:?}",
    //     (
    //         camera_transform.translation.x,
    //         camera_transform.translation.y
    //     ),
    //     (
    //         camera_projection.left,
    //         camera_projection.right,
    //         camera_projection.scale
    //     ),
    //     (screen_rect, screen_rect.width(), screen_rect.height())
    // );

    for tile_to_create in get_visible_tiles(
        screen_rect,
        terrain_texture_handle.tile_size,
        tile_builder.tile_scale,
    )
    .map(|position_tuple| position_tuple.into())
    {
        if let Some(_) = tile_builder.storage.get(&tile_to_create) {
            continue;
        }

        let new_tile = tile_builder.create(tile_to_create);

        println!("new_tile = {:?}", new_tile.transform);

        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: new_tile.tile.terrain as usize,
                    ..default()
                },
                texture_atlas: terrain_atlas_handle.clone(),
                transform: new_tile.transform,
                ..default()
            })
            .insert(Tile);
    }
}

// pub fn despawn_terrain(
//     mut commands: Commands,
//     world_frustum: Res<WorldViewFrustum>,
//     mut terrain_counter: ResMut<TerrainGenerator>,
//     mut tile_query: Query<(Entity, &Transform), With<Tile>>,
// ) {
//     let tile_mult = (world_frustum.terrain_tile_size * world_frustum.terrain_scale_factor) as f32;

//     for (entity, entity_transform) in tile_query.iter() {
//         if !world_frustum.is_viewing(&entity_transform.translation) {
//             let entity_tile_pos = (
//                 (entity_transform.translation.x / tile_mult) as i32,
//                 (entity_transform.translation.y / tile_mult) as i32,
//             );
//             if terrain_counter.storage.remove(&entity_tile_pos) {
//                 //println!("Limpei entidade {:?}", &entity);
//                 commands.entity(entity).despawn();
//             }
//         }
//     }
// }
