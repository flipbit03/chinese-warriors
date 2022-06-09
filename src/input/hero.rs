use bevy::{
    input::Input,
    prelude::{Commands, KeyCode, Query, Res, Transform, With, Without},
    render::camera::Camera2d,
};

use crate::{
    assets::config::structs::CwConfig,
    hero::structs::{Hero, HeroFacing},
};

use super::mouse_input::HeroMoveToInstruction;

pub fn hero_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    config: Res<CwConfig>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    mut query: Query<(&mut Hero, &mut Transform), Without<Camera2d>>,
) {
    let mut camera_transform = camera_query.single_mut();

    // TODO: Fix Diagonal Move Speed
    let move_speed = config.hero.move_speed * camera_transform.scale.x;

    let (mut hero, mut hero_transform) = query.single_mut();

    // If any key is pressed, remove currently existing MouseClick Hero Move Instruct
    let mut any_input = false;

    // Left
    if keyboard_input.just_pressed(KeyCode::A) {
        hero.facing = HeroFacing::Left;
        any_input = true;
    }

    if keyboard_input.pressed(KeyCode::A) {
        hero.facing = HeroFacing::Left;
        hero.walking = true;
        hero_transform.translation.x -= move_speed;
        any_input = true;
    }

    if keyboard_input.just_released(KeyCode::A) {
        hero.facing = HeroFacing::Left;
        hero.walking = false;
    }

    // Right
    if keyboard_input.just_pressed(KeyCode::D) {
        hero.facing = HeroFacing::Right;
        any_input = true;
    }

    if keyboard_input.pressed(KeyCode::D) {
        hero.facing = HeroFacing::Right;
        hero.walking = true;
        hero_transform.translation.x += move_speed;
        any_input = true;
    }

    if keyboard_input.just_released(KeyCode::D) {
        hero.facing = HeroFacing::Right;
        hero.walking = false;
    }

    // Up
    if keyboard_input.pressed(KeyCode::W) {
        hero.walking = true;
        hero_transform.translation.y += move_speed;
        any_input = true;
    }

    if keyboard_input.just_released(KeyCode::W) {
        hero.walking = false;
    }

    // Down
    if keyboard_input.pressed(KeyCode::S) {
        hero.walking = true;
        hero_transform.translation.y -= move_speed;
        any_input = true;
    }

    if keyboard_input.just_released(KeyCode::S) {
        hero.walking = false;
    }

    if any_input {
        commands.remove_resource::<HeroMoveToInstruction>();
    }
    camera_transform.translation = hero_transform.translation.clone();
}
