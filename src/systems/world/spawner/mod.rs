use super::tile::TileBuilder;
use crate::systems::resources::textures::TerrainTextures;
use crate::systems::world::tile::visibility::{get_screen_rect, get_visible_tiles};
use bevy::prelude::Transform;
use bevy::prelude::*;
use bevy::render::camera::Camera2d;

pub mod util;

#[derive(Component)]
pub struct Tile;

pub fn spawn_terrain(
    mut commands: Commands,
    terrain_textures: Res<TerrainTextures>,
    camera_query: Query<(&Transform, &OrthographicProjection), With<Camera2d>>,
    mut tile_builder: ResMut<TileBuilder>,
) {
    let (camera_transform, camera_projection) = camera_query.single();

    let screen_rect = get_screen_rect(camera_transform, camera_projection, 1.2);

    for tile_to_create in get_visible_tiles(
        screen_rect,
        terrain_textures.tile_size,
        tile_builder.tile_scale,
    )
    .map(|position_tuple| position_tuple.into())
    {
        if let Some(_) = tile_builder.storage.get(&tile_to_create) {
            continue;
        }

        let new_tile = tile_builder.create(tile_to_create);

        // Draw Base Terrain
        commands
        .spawn_bundle(SpriteBundle {
            texture: terrain_textures.base_terrain[new_tile.tile.terrain as usize].clone(),
            transform: new_tile.transform,
            ..Default::default()
        })
        .insert(Tile);

        // Draw Borders
        new_tile
            .tile
            .borders
            .into_iter()
            .for_each(|border_instruction| {
                for border_texture_index in border_instruction.get_texture_indexes() {
                    println!("Spawnando borda = {:?}", &border_instruction);
                    let border_terrain_transform = Transform {
                        translation: Vec3::new(
                            new_tile.transform.translation.x, 
                            new_tile.transform.translation.y, 
                            new_tile.transform.translation.z + 0.0001),
                        ..Default::default()
                    };

                    commands.spawn_bundle(SpriteBundle {
                        texture: terrain_textures.borders[border_instruction.terrain.clone() as usize -1]
                            [border_texture_index]
                            .clone(),
                        transform: border_terrain_transform,
                        ..Default::default()
                    });
                }
            });


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
