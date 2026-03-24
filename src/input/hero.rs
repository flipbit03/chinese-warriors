use bevy::prelude::*;

use crate::hero::{
    current_tile::MoveSpeed,
    structs::{Hero, HeroAction, HeroFacing},
};

use super::mouse_input::HeroMoveToInstruction;

pub fn hero_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    mut hero_query: Query<(&mut Hero, &mut Transform, &MoveSpeed), Without<Camera2d>>,
) {
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };
    let Ok((mut hero, mut hero_transform, hero_movespeed)) = hero_query.single_mut() else {
        return;
    };

    let up = keyboard_input.pressed(KeyCode::KeyW);
    let left = keyboard_input.pressed(KeyCode::KeyA);
    let down = keyboard_input.pressed(KeyCode::KeyS);
    let right = keyboard_input.pressed(KeyCode::KeyD);

    let mut dir = Vec2::new(0.0, 0.0);
    let diff = 2.0;

    match (up, down, left, right) {
        (true, true, true, true)
        | (true, true, false, false)
        | (false, false, true, true)
        | (false, false, false, false) => {
            hero.action = HeroAction::Idling;
            dir.x = 0.0;
            dir.y = 0.0;
        }

        (true, false, true, false) => {
            hero.action = HeroAction::Walking;
            hero.facing = HeroFacing::Left;
            dir.y = diff;
            dir.x = -diff;
        }

        (true, _, _, true) => {
            hero.action = HeroAction::Walking;
            hero.facing = HeroFacing::Right;
            dir.y = diff;
            dir.x = diff;
        }

        (_, true, true, _) => {
            hero.action = HeroAction::Walking;
            hero.facing = HeroFacing::Left;
            dir.y = -diff;
            dir.x = -diff;
        }

        (_, true, _, true) => {
            hero.action = HeroAction::Walking;
            hero.facing = HeroFacing::Right;
            dir.y = -diff;
            dir.x = diff;
        }

        (_, _, true, _) => {
            hero.action = HeroAction::Walking;
            hero.facing = HeroFacing::Left;
            dir.x = -diff;
        }

        (_, _, _, true) => {
            hero.action = HeroAction::Walking;
            hero.facing = HeroFacing::Right;
            dir.x = diff;
        }

        (true, _, _, _) => {
            hero.action = HeroAction::Walking;
            dir.y = diff;
        }

        (_, true, _, _) => {
            hero.action = HeroAction::Walking;
            dir.y = -diff;
        }
    }

    let any_input = up | down | left | right;
    if any_input {
        commands.remove_resource::<HeroMoveToInstruction>();
    }

    if dir.x != 0.0 && dir.y != 0.0 {
        dir.x = dir.x.abs().sqrt().copysign(dir.x);
        dir.y = dir.y.abs().sqrt().copysign(dir.y);
    }

    let move_speed = hero_movespeed.0 * camera_transform.scale.x;
    hero_transform.translation.x += dir.x / 2.0 * move_speed;
    hero_transform.translation.y += dir.y / 2.0 * move_speed;

    camera_transform.translation = hero_transform.translation;
}
