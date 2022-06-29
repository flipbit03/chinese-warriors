use bevy::{
    math::{Rect, Vec2},
    prelude::{OrthographicProjection, Transform},
};
use itertools::Itertools;

use super::{
    chunk::CHUNK_SIDE_SIZE,
    position::{ChunkPosition, TilePosition},
    traits::WidthHeight,
};

// pub fn get_screen_rect(
//     camera: &Camera,
//     camera_transform: &Transform,
//     camera_ortho: &OrthographicProjection,
// ) {
//     let input_normalized = Vec2::new(-1.0, 1.0);

//     // println!("mousepos={:?} ndc={:?}", mouse_position, ndc);
//     let ndc_to_world =
//         camera_transform.compute_matrix() * camera.projection_matrix.inverse();
//     let global_mouse_position =
//         ndc_to_world.project_point3(input_normalized.extend(-1.0)).truncate();

//     let screen_dimensions = get_screen_rect(
//         camera_transform,
//         camera_projection,
//         camera_projection.scale * 1.0,
//     );
// }

pub fn get_screen_rect(
    t: &Transform,
    p: &OrthographicProjection,
    scale: f32,
) -> Rect<f32> {
    Rect {
        left: t.translation.x - (p.left.abs() * scale),
        right: t.translation.x + (p.right.abs() * scale),
        top: t.translation.y + (p.top.abs() * scale),
        bottom: t.translation.y - (p.bottom.abs() * scale),
    }
}

pub fn get_visible_tiles(
    screen_dimensions: Rect<f32>,
    tile_size: Vec2,
    tile_scale: f32,
) -> impl Iterator<Item = TilePosition>
where
    Rect<f32>: WidthHeight,
{
    let horiz_divisor = tile_size.x * tile_scale;
    let vert_divisor = tile_size.y * tile_scale;

    let horizontal_tile_count = (screen_dimensions.width() / horiz_divisor).ceil() as i32;
    let horizontal_start = (screen_dimensions.left / horiz_divisor).floor() as i32;
    let horizontal_end = horizontal_start + horizontal_tile_count + 1;

    let vertical_tile_count = (screen_dimensions.height() / vert_divisor).ceil() as i32;
    let vertical_start = (screen_dimensions.bottom / vert_divisor).floor() as i32;
    let vertical_end = vertical_start + vertical_tile_count + 1;

    (horizontal_start..horizontal_end)
        .cartesian_product(vertical_start..vertical_end)
        .map(|x| x.into())
}

pub fn get_visible_chunks(
    screen_dimensions: Rect<f32>,
    tile_size: Vec2,
    tile_scale: f32,
) -> impl Iterator<Item = ChunkPosition>
where
    Rect<f32>: WidthHeight,
{
    get_visible_tiles(
        screen_dimensions,
        tile_size,
        tile_scale * CHUNK_SIDE_SIZE as f32,
    )
}
