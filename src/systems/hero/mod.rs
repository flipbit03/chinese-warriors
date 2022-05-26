use bevy::{prelude::*, render::camera::Camera2d};

use self::types::{Facing, Hero, HeroWalkCycleTimer};

use super::resources::textures::GuriTextureAtlas;
pub mod spawner;
pub mod types;

pub fn animate_hero(
    time: Res<Time>,
    guri_atlas: Res<GuriTextureAtlas>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut HeroWalkCycleTimer, &mut TextureAtlasSprite, &Hero), With<Hero>>,
) {
    let (mut timer, mut sprite, hero) = query.single_mut();

    timer.tick(time.delta());
    if timer.just_finished() {
        let texture_atlas = texture_atlases
            .get(guri_atlas.texture_handle.clone())
            .unwrap();
        match hero.facing {
            Facing::Right => {
                sprite.flip_x = true;
            }
            Facing::Left => {
                sprite.flip_x = false;
            }
        };

        if let true = hero.walking {
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        };
    };
}

pub fn hero_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    mut query: Query<(&mut Hero, &mut Transform), Without<Camera2d>>,
) {
    let mut camera_transform = camera_query.single_mut();

    // TODO: Fix Diagonal Move Speed
    let move_speed = 2.1;

    let (mut hero, mut hero_transform) = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::A) {
        hero.facing = Facing::Left;
    }

    if keyboard_input.pressed(KeyCode::A) {
        hero.facing = Facing::Left;
        hero.walking = true;
        hero_transform.translation.x -= move_speed;
    }

    if keyboard_input.just_released(KeyCode::A) {
        hero.facing = Facing::Left;
        hero.walking = false;
    }

    // Right
    if keyboard_input.just_pressed(KeyCode::D) {
        hero.facing = Facing::Right;
    }

    if keyboard_input.pressed(KeyCode::D) {
        hero.facing = Facing::Right;
        hero.walking = true;
        hero_transform.translation.x += move_speed;
    }

    if keyboard_input.just_released(KeyCode::D) {
        hero.facing = Facing::Right;
        hero.walking = false;
    }

    // Up
    if keyboard_input.pressed(KeyCode::W) {
        hero.walking = true;
        hero_transform.translation.y += move_speed;
    }

    if keyboard_input.just_released(KeyCode::W) {
        hero.walking = false;
    }

    // Down
    if keyboard_input.pressed(KeyCode::S) {
        hero.walking = true;
        hero_transform.translation.y -= move_speed;
    }

    if keyboard_input.just_released(KeyCode::S) {
        hero.walking = false;
    }

    camera_transform.translation = hero_transform.translation.clone();
}
