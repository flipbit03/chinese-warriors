use bevy::prelude::{App, Plugin};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

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
        app.add_enter_system(GameState::InGame, spawn_hero)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InGame)
                    .with_system(animate_hero)
                    .with_system(hero_current_tile_and_movespeed)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InGame)
                    .run_if_resource_exists::<HeroMoveToInstruction>()
                    .with_system(hero_movement_from_instruction)
                    .into(),
            );
    }
}
