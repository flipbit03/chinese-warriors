use bevy::{
    prelude::{Commands, Query, Res, Transform, With, Without},
    render::camera::Camera2d,
};

use crate::{
    assets::config::structs::CwConfig,
    hero::structs::{Hero, HeroFacing},
    input::mouse_input::HeroMoveToInstruction,
};

pub fn hero_movement_from_instruction(
    mut commands: Commands,
    optional_hmi: Option<Res<HeroMoveToInstruction>>,
    config: Res<CwConfig>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    mut query: Query<(&mut Hero, &mut Transform), Without<Camera2d>>,
) {
    let (mut hero, mut hero_transform) = query.single_mut();
    let mut camera_transform = camera_query.single_mut();

    // Make hero walk
    hero.walking = true;

    let hmi = optional_hmi.unwrap();

    // TODO: Fix Diagonal Move Speed
    let move_speed = config.hero.move_speed * camera_transform.scale.x;

    let hero_x = hero_transform.translation.x.round();
    let hero_y = hero_transform.translation.y.round();

    let distance_left_x = (hmi.0.x - hero_x).abs();
    let distance_left_y = (hmi.0.y - hero_y).abs();

    if distance_left_x > 1.0 {
        match hmi.0.x > hero_x {
            true => {
                hero_transform.translation.x = (hero_x + distance_left_x.min(move_speed)).round();
                hero.facing = HeroFacing::Right;
            }
            false => {
                hero_transform.translation.x = (hero_x - distance_left_x.min(move_speed)).round();
                hero.facing = HeroFacing::Left;
            }
        }
    }

    if distance_left_y > 1.0 {
        match hmi.0.y > hero_y {
            true => {
                hero_transform.translation.y = (hero_y + distance_left_y.min(move_speed)).round();
            }
            false => {
                hero_transform.translation.y = (hero_y - distance_left_y.min(move_speed)).round();
            }
        }
    }
    camera_transform.translation = hero_transform.translation.clone();

    if distance_left_x <= 1.0 && distance_left_y <= 1.0 {
        commands.remove_resource::<HeroMoveToInstruction>();
        hero.walking = false;
    }
}
