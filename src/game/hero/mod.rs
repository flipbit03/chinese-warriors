use bevy::prelude::*;
mod util;
use self::util::Facing;

use super::setup::{GlobalScaleFactor, GuriData};

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct Hero {
    pub facing: Facing,
    pub walking: bool,
}

impl Default for Hero {
    fn default() -> Self {
        Self {
            facing: Facing::Left,
            walking: false,
        }
    }
}

pub fn spawn_hero(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    guri_data: Res<GuriData>,
    global_scale: Res<GlobalScaleFactor>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let hero_transform = Transform {
        translation: Vec3::new(-0.0, 0.0, 1.0),
        ..Transform::from_scale(Vec3::splat(global_scale.factor))
    };

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: guri_data.texture_handle.clone(),
            transform: hero_transform,
            ..default()
        })
        .insert(Hero::default())
        .insert(AnimationTimer(Timer::from_seconds(0.07, true)));
}

pub fn animate_hero(
    time: Res<Time>,
    guri_data: Res<GuriData>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Hero), With<Hero>>,
) {
    let (mut timer, mut sprite, hero) = query.single_mut();

    timer.tick(time.delta());
    if timer.just_finished() {
        let texture_atlas = texture_atlases
            .get(guri_data.texture_handle.clone())
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
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut query: Query<(&mut Hero, &mut Transform), Without<Camera>>,
) {
    let mut camera_transform = camera_query.single_mut();

    let move_speed = 12.0;

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
