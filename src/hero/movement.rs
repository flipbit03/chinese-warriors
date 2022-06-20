use bevy::{
    prelude::{Commands, Query, Res, Transform, With, Without},
    render::camera::Camera2d,
};

use crate::{
    hero::structs::{Hero, HeroFacing},
    input::mouse_input::HeroMoveToInstruction,
};

use super::{current_tile::MoveSpeed, structs::HeroAction};

pub fn hero_movement_from_instruction(
    mut commands: Commands,
    hmi: Res<HeroMoveToInstruction>,
    camera_query: Query<&mut Transform, With<Camera2d>>,
    mut hero_query: Query<(&mut Hero, &mut Transform, &MoveSpeed), Without<Camera2d>>,
) {
    let (mut hero, mut hero_transform, hero_movespeed) = hero_query.single_mut();
    let camera_transform = camera_query.single();

    // We have an instruction to walk somewhere...
    hero.action = HeroAction::Walking;

    let hero_move_speed = hero_movespeed.0 * camera_transform.scale.x;

    let hero_x = hero_transform.translation.x.round();
    let hero_y = hero_transform.translation.y.round();

    let distance_left_x = (hmi.0.x - hero_x).abs();
    let distance_left_y = (hmi.0.y - hero_y).abs();

    if distance_left_x > 1.0 {
        match hmi.0.x > hero_x {
            true => {
                hero_transform.translation.x =
                    (hero_x + distance_left_x.min(hero_move_speed)).round();
                hero.facing = HeroFacing::Right;
            }
            false => {
                hero_transform.translation.x =
                    (hero_x - distance_left_x.min(hero_move_speed)).round();
                hero.facing = HeroFacing::Left;
            }
        }
    }

    if distance_left_y > 1.0 {
        match hmi.0.y > hero_y {
            true => {
                hero_transform.translation.y =
                    (hero_y + distance_left_y.min(hero_move_speed)).round();
            }
            false => {
                hero_transform.translation.y =
                    (hero_y - distance_left_y.min(hero_move_speed)).round();
            }
        }
    }

    if distance_left_x <= 1.0 && distance_left_y <= 1.0 {
        commands.remove_resource::<HeroMoveToInstruction>();
        hero.action = HeroAction::Idling;
    }
}
