use bevy::{prelude::*, window::WindowResized};

use crate::game::{hero::Hero, types::TilePosition};

#[derive(Debug)]
pub struct WorldFrustum {
    pub window_size: Vec2,
    pub rect: Rect<f32>,
    pub hero: Vec3,
    pub terrain_tile_size: f32,
    pub terrain_scale_factor: f32,
}

impl Default for WorldFrustum {
    fn default() -> Self {
        Self {
            window_size: Default::default(),
            rect: Default::default(),
            hero: Default::default(),
            terrain_tile_size: 32.0,
            terrain_scale_factor: 4.0,
        }
    }
}

impl WorldFrustum {
    pub fn get_visible_tiles(&self) -> Vec<TilePosition> {
        let divisor = self.terrain_tile_size * self.terrain_scale_factor;

        let horizontal_tile_count = (self.window_size.x / divisor).ceil() as i32;
        let horizontal_start = (self.rect.left / divisor).ceil() as i32 - 1; // -4
        let horizontal_end = horizontal_start + horizontal_tile_count + 1; // 3

        let vertical_tile_count = (self.window_size.y / divisor).ceil() as i32;
        let vertical_start = (self.rect.bottom / divisor).ceil() as i32 - 1;
        let vertical_end = vertical_start + vertical_tile_count + 1;

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
    mut window_resized_events: EventReader<WindowResized>,
    mut world_frustum: ResMut<WorldFrustum>,
    hero_query: Query<&Transform, With<Hero>>,
) {
    if let Some(s) = window_resized_events.iter().last() {
        world_frustum.window_size.x = s.width;
        world_frustum.window_size.y = s.height;
    }

    let hero_transform = hero_query.single();

    world_frustum.hero = hero_transform.translation.clone();

    let width_2 = world_frustum.window_size.x / 2.0;
    world_frustum.rect.left = hero_transform.translation.x - width_2;
    world_frustum.rect.right = hero_transform.translation.x + width_2;

    let height_2 = world_frustum.window_size.y / 2.0;
    world_frustum.rect.top = hero_transform.translation.y + height_2;
    world_frustum.rect.bottom = hero_transform.translation.y - height_2;
}
