use bevy::{
    math::{Rect, Vec2},
    prelude::{OrthographicProjection, Transform, Window},
};
use itertools::Itertools;

use super::{
    chunk::CHUNK_SIDE_SIZE,
    position::{ChunkPosition, TilePosition},
};

pub fn get_screen_rect(
    t: &Transform,
    p: &OrthographicProjection,
    window: &Window,
    extra_scale: f32,
) -> Rect {
    let half_width = (window.width() / 2.0) * p.scale * extra_scale;
    let half_height = (window.height() / 2.0) * p.scale * extra_scale;
    Rect::from_center_half_size(
        t.translation.truncate(),
        Vec2::new(half_width, half_height),
    )
}

pub fn get_visible_tiles(
    screen_dimensions: Rect,
    tile_size: Vec2,
    tile_scale: f32,
) -> impl Iterator<Item = TilePosition> {
    let horiz_divisor = tile_size.x * tile_scale;
    let vert_divisor = tile_size.y * tile_scale;

    let horizontal_tile_count = (screen_dimensions.width() / horiz_divisor).ceil() as i32;
    let horizontal_start = (screen_dimensions.min.x / horiz_divisor).floor() as i32;
    let horizontal_end = horizontal_start + horizontal_tile_count + 1;

    let vertical_tile_count = (screen_dimensions.height() / vert_divisor).ceil() as i32;
    let vertical_start = (screen_dimensions.min.y / vert_divisor).floor() as i32;
    let vertical_end = vertical_start + vertical_tile_count + 1;

    (horizontal_start..horizontal_end)
        .cartesian_product(vertical_start..vertical_end)
        .map(|x| x.into())
}

pub fn get_visible_chunks(
    screen_dimensions: Rect,
    tile_size: Vec2,
    tile_scale: f32,
) -> impl Iterator<Item = ChunkPosition> {
    get_visible_tiles(
        screen_dimensions,
        tile_size,
        tile_scale * CHUNK_SIDE_SIZE as f32,
    )
}
