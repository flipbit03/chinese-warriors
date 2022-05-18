use bevy::prelude::*;

use crate::game::{hero::Hero, types::TilePosition};

#[derive(Debug)]
pub struct Rect<T> {
    pub left: T,
    pub right: T,
    pub top: T,
    pub bottom: T,
}

impl<T: Default> Default for Rect<T> {
    fn default() -> Self {
        Self {
            left: Default::default(),
            right: Default::default(),
            top: Default::default(),
            bottom: Default::default(),
        }
    }
}

#[derive(Debug)]
/// Structure that defines the visible World rectangle
/// using World coordinates
pub struct WorldViewFrustum {
    pub window_size: Vec2,
    pub rect: Rect<f32>,
    pub hero: Vec3,
    pub terrain_tile_size: u32,
    pub terrain_scale_factor: u32,
}

impl Default for WorldViewFrustum {
    fn default() -> Self {
        Self {
            window_size: Default::default(),
            rect: Default::default(),
            hero: Default::default(),
            terrain_tile_size: 16,
            terrain_scale_factor: 2,
        }
    }
}

impl WorldViewFrustum {
    pub fn is_viewing(&self, pos: &Vec3) -> bool {
        self.rect.left < pos.x
            && self.rect.right > pos.x
            && self.rect.bottom < pos.y
            && self.rect.top > pos.y
    }

    pub fn get_visible_tiles(&self) -> Vec<TilePosition> {
        let divisor = (self.terrain_tile_size * self.terrain_scale_factor) as f32;

        let horizontal_tile_count = (self.window_size.x / divisor).ceil() as i32;
        let horizontal_start = (self.rect.left / divisor).ceil() as i32; // -4
        let horizontal_end = horizontal_start + horizontal_tile_count; // 3

        let vertical_tile_count = (self.window_size.y / divisor).ceil() as i32;
        let vertical_start = (self.rect.bottom / divisor).ceil() as i32;
        let vertical_end = vertical_start + vertical_tile_count;

        let mut visible_tile_array: Vec<TilePosition> = Vec::new();

        for horiz in horizontal_start..horizontal_end {
            for vert in vertical_start..vertical_end {
                visible_tile_array.push((horiz, vert));
            }
        }
        //println!("len(visible-tile) = {}", visible_tile_array.len());
        visible_tile_array
    }
}

pub fn update_world_frustum(
    windows: Res<Windows>,
    mut world_frustum: ResMut<WorldViewFrustum>,
    hero_query: Query<&Transform, With<Hero>>,
) {
    let w = windows.get_primary().unwrap();
    world_frustum.window_size.x = w.physical_width() as f32 * 1.2;
    world_frustum.window_size.y = w.physical_height() as f32 * 1.2;

    let hero_transform = hero_query.single();

    world_frustum.hero = hero_transform.translation.clone();

    let width_2 = world_frustum.window_size.x / 2.0;
    world_frustum.rect.left = hero_transform.translation.x - width_2;
    world_frustum.rect.right = hero_transform.translation.x + width_2;

    let height_2 = world_frustum.window_size.y / 2.0;
    world_frustum.rect.top = hero_transform.translation.y + height_2;
    world_frustum.rect.bottom = hero_transform.translation.y - height_2;
}
