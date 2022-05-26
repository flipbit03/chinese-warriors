use super::tile::TileBuilder;
use super::tile::position::TilePosition;
use crate::systems::resources::textures::TerrainTextures;
use crate::systems::world::tile::visibility::{get_screen_rect, get_visible_tiles};
use bevy::prelude::Transform;
use bevy::prelude::*;
use bevy::render::camera::Camera2d;

pub mod util;

#[derive(Component, Debug)]
pub struct DrawableTerrainMaterial(TilePosition);

pub fn spawn_terrain(
    mut commands: Commands,
    terrain_textures: Res<TerrainTextures>,
    camera_query: Query<(&Transform, &OrthographicProjection), With<Camera2d>>,
    mut tile_builder: ResMut<TileBuilder>,
) {
    let (camera_transform, camera_projection) = camera_query.single();

    let screen_rect = get_screen_rect(camera_transform, camera_projection, camera_projection.scale * 1.15);

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
        let new_tile_terrain_base = new_tile.tile.terrain.base.clone() as usize;

        // Draw Base Terrain
        commands
        .spawn_bundle(SpriteBundle {
            texture: terrain_textures.base_terrain[new_tile_terrain_base].clone(),
            transform: new_tile.transform,
            ..Default::default()
        })
        .insert(DrawableTerrainMaterial(new_tile.tile.position.clone()));

        // Draw Decoration
        if let Some(decoration_number) = new_tile.tile.terrain.decoration {
            commands
        .spawn_bundle(SpriteBundle {
            texture: terrain_textures.decorations[new_tile_terrain_base][decoration_number].clone(),
            transform: Transform {
                translation: Vec3::new(
                    new_tile.transform.translation.x, 
                    new_tile.transform.translation.y, 
                    new_tile.transform.translation.z + 0.00001),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(DrawableTerrainMaterial(new_tile.tile.position.clone()));
        }

        // Draw Borders
        new_tile
            .tile
            .borders
            .into_iter()
            .for_each(|border_instruction| {
                for border_texture_index in border_instruction.get_texture_indexes() {
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
                    })
                    .insert(DrawableTerrainMaterial(new_tile.tile.position.clone()));
                }
            });


    }
}

pub fn despawn_terrain(
    mut commands: Commands,
    tile_query: Query<(Entity, &DrawableTerrainMaterial)>,
    camera_query: Query<(&Transform, &OrthographicProjection), With<Camera2d>>,
    terrain_textures: Res<TerrainTextures>,
    mut tile_builder: ResMut<TileBuilder>,
) {

    let (camera_transform, camera_projection) = camera_query.single();

    let x : Vec<TilePosition> = get_visible_tiles(
        get_screen_rect(camera_transform, camera_projection, camera_projection.scale * 1.25),
        terrain_textures.tile_size,
        tile_builder.tile_scale,
    ).map(|i| { i.into() }).collect();

    for (tile_entity_id, tile_drawable_position) in tile_query.iter() {
        if !x.contains(&tile_drawable_position.0) {
            println!("Despawning Tile={:?} Drawable={:?}", tile_entity_id, tile_drawable_position);
            commands.entity(tile_entity_id).despawn();
            tile_builder.storage.remove(&tile_drawable_position.0);
        }
    }
}
