use bevy::prelude::*;

use crate::{app::GameState, input::mouse_input::HeroMoveToInstruction};

use self::{
    animation::animate_hero, current_tile::hero_current_tile_and_movespeed,
    movement::hero_movement_from_instruction, spawner::spawn_hero,
};

pub mod animation;
pub mod current_tile;
pub mod movement;
pub mod spawner;
pub mod structs;

pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_hero)
            .add_systems(
                Update,
                (animate_hero, hero_current_tile_and_movespeed)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                hero_movement_from_instruction
                    .run_if(in_state(GameState::InGame))
                    .run_if(resource_exists::<HeroMoveToInstruction>),
            );
    }
}
