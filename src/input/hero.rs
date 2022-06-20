use bevy::{
    input::Input,
    math::Vec2,
    prelude::{Commands, KeyCode, Query, Res, Transform, With, Without},
    render::camera::Camera2d,
};

use crate::hero::{
    current_tile::MoveSpeed,
    structs::{Hero, HeroAction, HeroFacing},
};

use super::mouse_input::HeroMoveToInstruction;

pub fn hero_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    mut hero_query: Query<(&mut Hero, &mut Transform, &MoveSpeed), Without<Camera2d>>,
) {
    let mut camera_transform = camera_query.single_mut();

    let (mut hero, mut hero_transform, hero_movespeed) = hero_query.single_mut();

    let up = keyboard_input.pressed(KeyCode::W);
    let left = keyboard_input.pressed(KeyCode::A);
    let down = keyboard_input.pressed(KeyCode::S);
    let right = keyboard_input.pressed(KeyCode::D);

    let mut dir = Vec2::new(0.0, 0.0);
    let diff = 2.0;

    match (up, down, left, right) {
        // Nothing pressed, and "cancelling" combinations
        (true, true, true, true)
        | (true, true, false, false)
        | (false, false, true, true)
        | (false, false, false, false) => {
            hero.action = HeroAction::Idling;
            dir.x = 0.0;
            dir.y = 0.0;
        }

        // Up Left
        (true, false, true, false) => {
            hero.action = HeroAction::Walking;
            hero.facing = HeroFacing::Left;
            dir.y = diff;
            dir.x = -diff;
        }

        // Up Right
        (true, _, _, true) => {
            hero.action = HeroAction::Walking;
            hero.facing = HeroFacing::Right;
            dir.y = diff;
            dir.x = diff;
        }

        // Down Left
        (_, true, true, _) => {
            hero.action = HeroAction::Walking;
            hero.facing = HeroFacing::Left;
            dir.y = -diff;
            dir.x = -diff;
        }

        // Down Right
        (_, true, _, true) => {
            hero.action = HeroAction::Walking;
            hero.facing = HeroFacing::Right;
            dir.y = -diff;
            dir.x = diff;
        }

        // Left
        (_, _, true, _) => {
            hero.action = HeroAction::Walking;
            hero.facing = HeroFacing::Left;
            dir.x = -diff;
        }

        // Right
        (_, _, _, true) => {
            hero.action = HeroAction::Walking;
            hero.facing = HeroFacing::Right;
            dir.x = diff;
        }

        // Up
        (true, _, _, _) => {
            hero.action = HeroAction::Walking;
            dir.y = diff;
        }

        // Down
        (_, true, _, _) => {
            hero.action = HeroAction::Walking;
            dir.y = -diff;
        }
    }

    // If any key is pressed, remove currently existing MouseClick Hero Move Instruct
    let any_input = up | down | left | right;
    if any_input {
        hero.action = HeroAction::Idling;
        commands.remove_resource::<HeroMoveToInstruction>();
    }

    if dir.x != 0.0 && dir.y != 0.0 {
        dir.x = dir.x.abs().sqrt().copysign(dir.x);
        dir.y = dir.y.abs().sqrt().copysign(dir.y);
    }

    let move_speed = hero_movespeed.0 * camera_transform.scale.x;
    hero_transform.translation.x =
        hero_transform.translation.x + (dir.x / 2.0 * move_speed);
    hero_transform.translation.y =
        hero_transform.translation.y + (dir.y / 2.0 * move_speed);

    camera_transform.translation = hero_transform.translation.clone();
}
